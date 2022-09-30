//! [`Task`] for execution by a [`platform::dart::executor`].
//!
//! [`platform::dart::executor`]: crate::platform::executor
extern crate alloc;

use std::{
    cell::{Cell, RefCell},
    fmt,
    future::Future,
    mem::ManuallyDrop,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use flutter_rust_bridge::{StreamSink, support::WireSyncReturnData};

use crate::api::{FutureEvent};

pub type LocalBoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + 'a>>;

pub fn my_spawn(event: StreamSink<FutureEvent>, future: impl Future<Output = ()> + 'static) {
    let res: Rc<RefCell<Option<Rc<Task>>>> = Rc::new(RefCell::new(None));
    let raw = Rc::into_raw(res) as _;
    event.add(FutureEvent::Init(raw));
    let task = Task::spawn(Box::pin(future), event);
    let res = unsafe { Rc::<RefCell<Option<Rc<Task>>>>::from_raw(raw as _) };
    res.as_ref().borrow_mut().replace(task);
    drop(Rc::into_raw(res));
}

/// Inner [`Task`]'s data.
struct Inner {
    /// An actual [`Future`] that this [`Task`] is driving.
    ///
    /// [`Future`]: std::future::Future
    future: LocalBoxFuture<'static, ()>,

    /// Handle for waking up this [`Task`].
    waker: Waker,
}

impl Drop for Inner {
    fn drop(&mut self) {
        // println!("DROP Inner");
    }
}

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inner")
            .field("waker", &self.waker)
            .finish_non_exhaustive()
    }
}

/// Wrapper for a [`Future`] that can be polled by an external single threaded
/// Dart executor.
///
/// [`Future`]: std::future::Future
pub struct Task {
    /// [`Task`]'s inner data containing an actual [`Future`] and its
    /// [`Waker`]. Dropped on the [`Task`] completion.
    ///
    /// [`Future`]: std::future::Future
    inner: RefCell<Option<Inner>>,
    /// Indicates whether there is a [`Poll::Pending`] awake request of this
    /// [`Task`].
    is_scheduled: Cell<bool>,
    cb: StreamSink<FutureEvent>,
}

impl Drop for Task {
    fn drop(&mut self) {
        self.cb.close();
    }
}

impl Task {
    /// Spawns a new [`Task`] that will drive the given [`Future`].
    ///
    /// [`Future`]: std::future::Future
    pub fn spawn(future: LocalBoxFuture<'static, ()>, event: StreamSink<FutureEvent>) -> Rc<Self> {
        let this = Rc::new(Self {
            inner: RefCell::new(None),
            cb: event,
            is_scheduled: Cell::new(false),
        });

        let waker = unsafe { Waker::from_raw(Self::into_raw_waker(Rc::clone(&this))) };

        drop(this.inner.borrow_mut().replace(Inner { future, waker }));

        Self::wake_by_ref(&this);
        this
    }

    /// Polls the underlying [`Future`].
    ///
    /// Polling after [`Future`]'s completion is no-op.
    ///
    /// [`Future`]: std::future::Future
    pub fn poll(&self) -> Poll<()> {
        let mut borrow = self.inner.borrow_mut();

        // Just ignore poll request if the `Future` is completed.
        let inner = match borrow.as_mut() {
            Some(inner) => inner,
            None => return Poll::Ready(()),
        };

        let poll = {
            let mut cx = Context::from_waker(&inner.waker);
            inner.future.as_mut().poll(&mut cx)
        };
        self.is_scheduled.set(false);

        // Cleanup resources if future is ready.
        if poll.is_ready() {
            *borrow = None;
        }

        poll
    }

    /// Calls the [`task_wake()`] function by the provided reference if this
    /// [`Task`] s incomplete and there are no [`Poll::Pending`] awake requests
    /// already.
    fn wake_by_ref(this: &Rc<Self>) {
        if !this.is_scheduled.replace(true) {
            this.cb.add(FutureEvent::Wake);
        }
    }

    /// Pretty much a copy of [`std::task::Wake`] implementation but for
    /// `Rc<?Send + ?Sync>` instead of `Arc<Send + Sync>` since we are sure
    /// that everything will run on a single thread.
    fn into_raw_waker(this: Rc<Self>) -> RawWaker {
        #![allow(clippy::missing_docs_in_private_items)]

        // Refer to `RawWakerVTable::new()` documentation for better
        // understanding of what the following functions do.

        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr.cast::<Task>()));
            Task::into_raw_waker(Rc::clone(&(*ptr)))
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = Rc::from_raw(ptr.cast::<Task>());
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr.cast::<Task>()));
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(Rc::from_raw(ptr.cast::<Task>()));
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this).cast::<()>(), &VTABLE)
    }
}

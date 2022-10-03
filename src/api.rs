
extern crate lazy_static;
use std::{cell::RefCell, rc::Rc, sync::{atomic::{AtomicU64, Ordering}, Mutex}, future::Future, time::Duration};

use flutter_rust_bridge::{StreamSink, SyncReturn, };
use once_cell::sync::OnceCell;

use async_std::task;

use crate::executor::{Task};
use lazy_static::lazy_static;

pub static WAKER: OnceCell<StreamSink<FutureEvent>> = OnceCell::new();
lazy_static! {
    pub static ref PREWAKERS: Mutex<Vec<FutureEvent>> = Mutex::new(vec![]);
}

/// Counter used to generate unique IDs.
static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Returns a next unique ID.
pub(crate) fn next_id() -> u64 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}


#[derive(Clone, Debug)]
pub enum FutureEvent {
    Init(u64),
    Wake
}

fn my_spawn(future: impl Future<Output = ()> + 'static) {
    Task::spawn(Box::pin(future));
}

pub fn spawn() -> SyncReturn<bool> {
    let id = next_id();
    my_spawn(
        async move {
        task::sleep(Duration::from_secs(10)).await;
        println!("DONE {id}");


    });
    SyncReturn(true)
}

pub fn poll(raw: u64) -> SyncReturn<bool> {
    let task: Rc<Task> = unsafe { Rc::from_raw(raw as _) };
    let res = task.poll().is_ready();
    SyncReturn(res)
}

pub fn init_executor(cb: StreamSink<FutureEvent>) ->  anyhow::Result<()> {
    println!("RAZ");
    let mut lock = PREWAKERS.lock().unwrap();
    while let Some(event) = lock.pop() {
        cb.add(event);
    }
    WAKER.set(cb);
    Ok(())
}


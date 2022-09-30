use std::{cell::RefCell, rc::Rc};

use flutter_rust_bridge::{StreamSink, SyncReturn, };

use crate::executor::{my_spawn, Task};

pub enum FutureEvent {
    Init(u64),
    Wake,
}

pub fn spawn(cb: StreamSink<FutureEvent>) -> anyhow::Result<()> {
    my_spawn(
        cb,
        async move {
            // task::sleep(Duration::from_millis(1)).await;
            println!("DONE");
        },
    );
    Ok(())
}


pub fn poll(raw: u64) -> SyncReturn<bool> {
    let task: Rc<RefCell<Option<Rc<Task>>>> = unsafe { Rc::from_raw(raw as _) };
    let res = task.borrow_mut().as_mut().unwrap().poll().is_ready();
    drop(Rc::into_raw(task));
    SyncReturn(res)
}

pub fn drop_future(raw: u64) -> SyncReturn<bool> {
    drop(unsafe { Rc::<RefCell<Option<Rc<Task>>>>::from_raw(raw as _) });
    SyncReturn(true)
}


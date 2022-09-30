#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.48.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

fn wire_spawn_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "spawn",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| spawn(task_callback.stream_sink()),
    )
}
fn wire_poll_impl(raw: impl Wire2Api<u64> + UnwindSafe) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "poll",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_raw = raw.wire2api();
            Ok(poll(api_raw))
        },
    )
}
fn wire_drop_future_impl(raw: impl Wire2Api<u64> + UnwindSafe) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "drop_future",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_raw = raw.wire2api();
            Ok(drop_future(api_raw))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for FutureEvent {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Init(field0) => vec![0.into_dart(), field0.into_dart()],
            Self::Wake => vec![1.into_dart()],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for FutureEvent {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

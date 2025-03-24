use crate::bindings::*;

use hyperlight_common::resource::BorrowedResourceGuard;

use crate::state::MyState;

pub enum MyPollable {
    AlwaysReady,
}

impl wasi::io::poll::Pollable for MyState {
    type T = MyPollable;
    fn block(&mut self, _self: BorrowedResourceGuard<MyPollable>) {
        ()
    }
}

impl wasi::io::Poll for MyState {
}

use crate::bindings::*;
use crate::cli;

use hyperlight_common::resource::BorrowedResourceGuard;

pub enum Void {}

impl wasi::sockets::InstanceNetwork<()> for () {
    fn instance_network(&mut self) -> ()  {
        ()
    }
}

impl wasi::io::error::Error for Void {
    type T = Void;

}

impl wasi::io::Error for Void {
}

impl wasi::cli::Exit for Void {
    fn exit(&mut self, _status: Result<(), ()>) { match *self {} }
}

impl wasi::filesystem::types::Descriptor<wasi::clocks::wall_clock::Datetime, cli::OutputStream> for Void {
    type T = Void;
    fn write_via_stream(
        &mut self,
        _self: BorrowedResourceGuard<Void>,
        _offset: u64
    ) -> Result<cli::OutputStream, wasi::filesystem::types::ErrorCode> { match *self {} }
    fn r#append_via_stream(
        &mut self,
        _self: BorrowedResourceGuard<Void>,
    ) -> std::result::Result<cli::OutputStream, wasi::filesystem::types::ErrorCode> {
        match *self {}
    }
    fn r#get_type(
        &mut self,
        _self: BorrowedResourceGuard<Void>,
    ) -> std::result::Result<
        wasi::filesystem::types::DescriptorType,
        wasi::filesystem::types::ErrorCode
        > {
        match *self {}
    }
    fn r#stat(
        &mut self,
        _self: BorrowedResourceGuard<Void>,
    ) -> std::result::Result<wasi::filesystem::types::DescriptorStat<wasi::clocks::wall_clock::Datetime>, wasi::filesystem::types::ErrorCode> {
        match *self {}
    }

}

impl wasi::clocks::WallClock for Void {
}

impl wasi::filesystem::Types<wasi::clocks::wall_clock::Datetime, Void, cli::OutputStream> for Void {
    fn filesystem_error_code(
        &mut self,
        _err: BorrowedResourceGuard<Void>,
    ) -> std::option::Option<wasi::filesystem::types::ErrorCode> {
        match *self {}
    }
}

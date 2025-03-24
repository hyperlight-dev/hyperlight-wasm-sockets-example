#![allow(refining_impl_trait)]

extern crate alloc;

mod bindings;
use bindings::*;

mod state;
use state::MyState;

mod stub;
use stub::Void;

mod poll;

mod udp;

mod cli;
use cli::*;

impl wasi::sockets::Network for MyState {
}

impl root::component::RootImports for MyState {
    type UdpCreateSocket = MyState;
    fn udp_create_socket(&mut self) -> &mut Self { self }

    type Network = MyState;
    fn network(&mut self) -> &mut Self { self }

    type Poll = MyState;
    fn r#poll(&mut self) -> &mut Self { self }

    type Udp = MyState;
    fn r#udp(&mut self) -> &mut Self  { self }

    type InstanceNetwork = ();
    fn r#instance_network(&mut self) -> () { () }

    type Error = Void;
    fn r#error(&mut self) -> Void { todo!() }

    type Streams = Streams;
    fn r#streams(&mut self) -> Streams { Streams {} }

    type Environment = Environment;
    fn r#environment(&mut self) -> Environment { Environment {} }

    type Exit = Void;
    fn r#exit(&mut self) -> Void { todo!() }

    type Stdin = Streams;
    fn r#stdin(&mut self) -> Streams { Streams {} }

    type Stdout = Streams;
    fn r#stdout(&mut self) -> Streams { Streams {} }

    type Stderr = Streams;
    fn r#stderr(&mut self) -> Streams { Streams {} }

    type WallClock = Void;
    fn r#wall_clock(&mut self) -> Void { todo!() }

    type Types = Void;
    fn r#types(&mut self) -> Void { todo!() }

    type Preopens = Preopens;
    fn r#preopens(&mut self) -> Preopens { Preopens {} }
}

use crate::bindings::wasi::cli::Run;

fn main() {
    let state = MyState::new();

    // Set the maximum allowed resource consumption for the sandbox
    let mut sb: hyperlight_wasm::ProtoWasmSandbox =
        hyperlight_wasm::SandboxBuilder::new()
            .with_guest_input_buffer_size(5000000)
            .with_guest_heap_size(10000000)
            .with_guest_panic_context_buffer_size(10000000)
            .with_guest_stack_size(10000000)
            .with_guest_function_call_max_execution_time_millis(0)
            .build()
            .unwrap();
    // Provide the imports we just implemented (capturing `state`),
    // returning the resource table used to keep track of host
    // resources handed to the component
    let rt = crate::bindings::register_host_functions(&mut sb, state);
    // Initialise the Wasm engine inside the sandbox
    let sb = sb.load_runtime().unwrap();
    // Load the component we actually want to execute
    let sb = sb.load_module("echo.bin").unwrap();
    // Wrap up the sandbox and the resources to get something which
    // implements the wasi::cli::Run trait representing the exported
    // run() instance
    let mut wrapped = bindings::RootSandbox { sb, rt };
    // Run the sandbox!
    let run_inst = root::component::RootExports::run(&mut wrapped);
    let _ = run_inst.run();
}

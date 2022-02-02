// #[macro_use] extern crate rustler;
// #[macro_use] extern crate lazy_static;

// Disabled for now, but still
// wondering why rustler generated this in the first place
// #[macro_use] extern crate rustler_codegen;

use rustler::{NifResult, Term, Env, Encoder};
use rustler::types::Atom;
extern crate pact_mock_server;
extern crate libc;

// use pact_mock_server::create_mock_server;// as create_mock_server_a;
use pact_ffi::mock_server::pactffi_create_mock_server;
use pact_ffi::mock_server::pactffi_mock_server_mismatches;
use pact_ffi::mock_server::pactffi_mock_server_matched;
use pact_mock_server::write_pact_file;
use pact_mock_server::WritePactFileErr;
use pact_ffi::mock_server::pactffi_cleanup_mock_server;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        mock_server_failed_to_start,
        invalid_pact_json,
        io_error,
        no_mock_server_running_on_port,
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::init!(
    "Elixir.PactElixir.RustPactMockServerFacade",
    [
        create_mock_server_call,
        mock_server_mismatches_call,
        mock_server_matched_call,
        write_pact_file_call,
        cleanup_mock_server_call
    ]
);

#[rustler::nif(name = "create_mock_server")]
fn create_mock_server_call<'a>(env: Env<'a>, pact_json_string: String, address_string: String) -> NifResult<Term<'a>> {
    let pact_json: i8 = pact_json_string.as_str().as_ptr() as i8;
    let address: i8 = address_string.as_str().as_ptr() as i8;
    let port = pactffi_create_mock_server(&pact_json, &address, false);

    match port {
        port if port >= 0 => Ok((atoms::ok(), port).encode(env)),
        port if port < 0 =>
            Ok( (atoms::error(),  atoms::mock_server_failed_to_start(), port).encode(env) ),
        _ => Ok( (atoms::error(), atoms::error(), "RustPactMockServerFacade: Unknown error while starting mock server").encode(env) )
    }
}

#[rustler::nif(name = "mock_server_mismatches")]
fn mock_server_mismatches_call(port:i32) -> NifResult<(Atom, Option<String>)> {
    Ok((atoms::ok(), pactffi_mock_server_mismatches(port)))
}

#[rustler::nif(name = "mock_server_matched")]
fn mock_server_matched_call(port:i32) -> NifResult<(Atom,bool)> {
    Ok((atoms::ok(), pactffi_mock_server_matched(port)))
}

#[rustler::nif(name = "write_pact_file")]
fn write_pact_file_call<'a>(env: Env<'a>, port:i32, dir_path:String) -> NifResult<Term<'a>> {
    match write_pact_file(port, Some(dir_path)) {
        Ok(()) =>
            Ok((atoms::ok()).encode(env)),
        Err(WritePactFileErr::IOError) =>
            Ok( (atoms::error(), atoms::io_error()).encode(env) ),
        Err(WritePactFileErr::NoMockServer) =>
            Ok((atoms::error(), atoms::no_mock_server_running_on_port()).encode(env))
    }
}

#[rustler::nif(name = "cleanup_mock_server")]
fn cleanup_mock_server_call(port:i32) -> NifResult<(Atom,bool)> {
    Ok((atoms::ok(), pactffi_cleanup_mock_server(port)))
}

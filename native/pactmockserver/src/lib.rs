#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;

// Disabled for now, but still
// wondering why rustler generated this in the first place
// #[macro_use] extern crate rustler_codegen;

use rustler::{Env, Term, NifResult, Encoder};
extern crate pact_mock_server;
extern crate libc;

use pact_mock_server::create_mock_server;
use pact_mock_server::MockServerError;
use pact_mock_server::mock_server_mismatches;
use pact_mock_server::mock_server_matched;
use pact_mock_server::write_pact_file;
use pact_mock_server::WritePactFileErr;
use pact_mock_server::cleanup_mock_server_ffi;
mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom mock_server_failed_to_start;
        atom invalid_pact_json;
        atom io_error;
        atom no_mock_server_running_on_port;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.PactElixir.RustPactMockServerFacade",
    [
        ("create_mock_server", 2, create_mock_server_call),
        ("mock_server_mismatches", 1, mock_server_mismatches_call),
        ("mock_server_matched", 1, mock_server_matched_call),
        ("write_pact_file", 2, write_pact_file_call),
        ("cleanup_mock_server", 1, cleanup_mock_server_call)
    ],
    None
}

fn create_mock_server_call<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let pact_json: &str = try!(args[0].decode());
    let port_arg: i32 = try!(args[1].decode());

    match create_mock_server(pact_json, port_arg) {
        Ok(port) =>
            Ok((atoms::ok(), port).encode(env)),
        Err(MockServerError::MockServerFailedToStart) =>
            Ok((atoms::error(), atoms::mock_server_failed_to_start()).encode(env)),
        Err(MockServerError::InvalidPactJson) =>
            Ok((atoms::error(), atoms::invalid_pact_json()).encode(env))
    }
}

fn mock_server_mismatches_call<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let port: i32 = try!(args[0].decode());
    Ok((atoms::ok(), mock_server_mismatches(port)).encode(env))
}

fn mock_server_matched_call<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let port: i32 = try!(args[0].decode());

    let matched: bool = mock_server_matched(port);

    Ok((atoms::ok(), matched).encode(env))
}

fn write_pact_file_call<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let port: i32 = try!(args[0].decode());
    let dir_path: String = try!(args[1].decode());

    match write_pact_file(port, Some(dir_path)) {
        Ok(_result) =>
            Ok((atoms::ok()).encode(env)),
        Err(WritePactFileErr::IOError) =>
            Ok((atoms::error(), atoms::io_error()).encode(env)),
        Err(WritePactFileErr::NoMockServer) =>
            Ok((atoms::error(), atoms::no_mock_server_running_on_port()).encode(env))
    }
}

fn cleanup_mock_server_call<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let port: i32 = try!(args[0].decode());

    Ok((atoms::ok(), cleanup_mock_server_ffi(port)).encode(env))
}


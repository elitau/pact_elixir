// #[macro_use] extern crate rustler;
// #[macro_use] extern crate lazy_static;

// Disabled for now, but still
// wondering why rustler generated this in the first place
// #[macro_use] extern crate rustler_codegen;

// use rustler::{Env, Term, NifResult, Encoder}; // MON30AUG210852
use rustler::{NifResult}; // MON30AUG210852
extern crate pact_mock_server;
extern crate libc;

// use pact_mock_server::create_mock_server;// as create_mock_server_a;
use pact_mock_server::create_mock_server;
use pact_mock_server::MockServerError; // unused because of the way the fuctions are being implemented.
use pact_mock_server::mock_server_mismatches;
use pact_mock_server::mock_server_matched;
use pact_mock_server::write_pact_file;
use pact_mock_server::WritePactFileErr; 
use pact_mock_server::cleanup_mock_server_ffi;
// use pact_mock_server::cleanup_mock_server;
// mod atoms {
//     rustler::atoms! {
//         atom ok;
//         atom error;
//         atom mock_server_failed_to_start;
//         atom invalid_pact_json;
//         atom io_error;
//         atom no_mock_server_running_on_port;
//         //atom __true__ = "true";
//         //atom __false__ = "false";
//     }
// }

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

// rustler_export_nifs! {
//     "Elixir.PactElixir.RustPactMockServerFacade",
//     [
//         ("create_mock_server", 2, create_mock_server_call),
//         ("mock_server_mismatches", 1, mock_server_mismatches_call),
//         ("mock_server_matched", 1, mock_server_matched_call),
//         ("write_pact_file", 2, write_pact_file_call),
//         ("cleanup_mock_server", 1, cleanup_mock_server_call)
//     ],
//     None
// }

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
fn create_mock_server_call(pact_json:&str, port_arg:i32) -> NifResult<i32> {
// fn create_mock_server(args: &[Term]) -> NifResult<i32> {
    // let pact_json: &str = r#try!(args[0].decode());
    // let port_arg: i32 = r#try!(args[1].decode());
    // let pact_json: &str = (args[0].decode())?;
    // let port_arg: i32 = (args[1].decode())?;
    let port = create_mock_server(pact_json, port_arg);
    // port

    match port {
        Ok(port) => {
            Ok(port)
        }
        Err(MockServerError::MockServerFailedToStart) =>
            NifResult::Err( rustler::error::Error::Atom("mock_server_failed_to_start")),
        Err(MockServerError::InvalidPactJson) => 
            NifResult::Err( rustler::error::Error::Atom("invalid_pact_json"))
    }
}

#[rustler::nif(name = "mock_server_mismatches")]
// fn mock_server_mismatches<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
//     let port: i32 = (args[0].decode())?;
//     Ok((atoms::ok(), mock_server_mismatches(port)).encode(env))
// }
fn mock_server_mismatches_call(port:i32) -> Option<String> {
    mock_server_mismatches(port)
}

#[rustler::nif(name = "mock_server_matched")]
// fn mock_server_matched<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
//     let port: i32 = args[0].decode();

//     let matched: bool = mock_server_matched(port);

//     Ok((atoms::ok(), matched).encode(env))
// }
fn mock_server_matched_call(port:i32) -> bool {

    // let matched: bool = mock_server_matched(port);

    // Ok(matched)
    mock_server_matched(port)
}

// fn write_pact_file<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
//     let port: i32 = (args[0].decode())?;
//     let dir_path: String = (args[1].decode())?;

//     match write_pact_file(port, Some(dir_path)) {
//         Ok(_result) =>
//             Ok((atoms::ok()).encode(env)),
//         Err(WritePactFileErr::IOError) =>
//             Ok((atoms::error(), atoms::io_error()).encode(env)),
//         Err(WritePactFileErr::NoMockServer) =>
//             Ok((atoms::error(), atoms::no_mock_server_running_on_port()).encode(env))
//     }
// }
#[rustler::nif(name = "write_pact_file")]
fn write_pact_file_call(port:i32, dir_path:String) -> NifResult<()> {
    match write_pact_file(port, Some(dir_path)) {
        Ok(()) =>
            Ok(()),
        Err(WritePactFileErr::IOError) =>
            NifResult::Err( rustler::error::Error::Atom("io_error")),
        Err(WritePactFileErr::NoMockServer) =>
            NifResult::Err( rustler::error::Error::Atom("no_mock_server_running_on_port"))
    }
}

// fn cleanup_mock_server<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
//     let port: i32 = (args[0].decode())?;

//     Ok((atoms::ok(), cleanup_mock_server_ffi(port)).encode(env))
// }
#[rustler::nif(name = "cleanup_mock_server")]
fn cleanup_mock_server_call(port:i32) -> bool {
    cleanup_mock_server_ffi(port)
}
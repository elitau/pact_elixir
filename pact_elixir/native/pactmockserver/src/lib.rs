#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate pact_mock_server;
extern crate libc;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use rustler::types::binary::NifBinary;
use pact_mock_server::create_mock_server;
use libc::c_char;
use std::io;
use std::ffi::CString;
mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.PactElixir.PactMockServer",
    [("create_mock_server", 2, create_mock_server_call)],
    None
}

fn add<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn create_mock_server_call<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let arg1: String = try!(args[0].decode());
    let arg2: i32 = try!(args[1].decode());

    let s = CString::new(arg1).unwrap();

    let result = create_mock_server(s.as_ptr(), arg2);

    Ok((atoms::ok(), result).encode(env))
}

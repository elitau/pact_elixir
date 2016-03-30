#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;
extern crate libc;
extern crate libpact_v1_matching;
extern crate rustc_serialize;
extern crate env_logger;

use libc::{c_char, int16_t};
use std::ffi::CStr;
use std::str;
use libpact_v1_matching::models::Pact;
use rustc_serialize::json::Json;

pub struct MockServer;

#[no_mangle]
pub extern fn create_mock_server(pact_str: *const c_char) -> int16_t {
    env_logger::init().unwrap();

    let c_str = unsafe {
        if pact_str.is_null() {
            error!("Got a null pointer instead of pact json");
            return -1;
        }
        CStr::from_ptr(pact_str)
    };

    let pact_json = str::from_utf8(c_str.to_bytes()).unwrap();
    let result = Json::from_str(pact_json);
    match result {
        Ok(pact_json) => {
            p!(pact_json);
            let pact = Pact::from_json(&pact_json);
            p!(pact);
            0
        },
        Err(err) => {
            error!("Could not parse pact json: {}", err);
            -2
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

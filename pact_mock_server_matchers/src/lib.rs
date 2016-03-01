extern crate rustc_serialize;

pub mod model;

use model::{Request, Response};

pub enum Mismatch {

}

pub fn match_request(expected: &Request, actual: &Request) -> Vec<Mismatch> {
    Vec::new()
}

pub fn match_response(expected: &Response, actual: &Response) -> Vec<Mismatch> {
    Vec::new()
}

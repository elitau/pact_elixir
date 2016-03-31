#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;
extern crate libc;
#[macro_use] extern crate libpact_v1_matching;
extern crate rustc_serialize;
extern crate env_logger;
#[macro_use] extern crate hyper;

use libc::{c_char, int32_t};
use std::ffi::CStr;
use std::str;
use libpact_v1_matching::models::{Pact, Interaction, Request, Response, OptionalBody};
use libpact_v1_matching::models::parse_query_string;
use libpact_v1_matching::Mismatch;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::collections::{BTreeMap, HashMap};

use std::thread;
use std::sync::RwLock;
use std::sync::mpsc::channel;
use std::io::{Read, Write};
use hyper::server::Server;
use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType, AccessControlAllowOrigin, ContentLength};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::uri::RequestUri;

#[derive(Debug, Clone, PartialEq)]
pub struct MockServer {
    pub port: i32
}

lazy_static! {
    static ref MOCK_SERVERS: RwLock<BTreeMap<i32, MockServer>> = RwLock::new(BTreeMap::new());
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    RequestMatch(Response),
    RequestMismatch(Vec<(Interaction, Vec<Mismatch>)>),
    RequestNotFound(Request)
}

fn match_request(req: &Request, interactions: &Vec<Interaction>) -> MatchResult {
    let list = interactions.clone();
    let list: Vec<Interaction> = list.iter().filter(|i| {
        i.request.method == req.method && i.request.path == req.path
    }).map(|i| i.clone() ).collect();
    if list.is_empty() {
        MatchResult::RequestNotFound(req.clone())
    } else {
        let matches: Vec<(Interaction, Vec<Mismatch>)> = list.iter().map(|i| {
            let mismatches = libpact_v1_matching::match_request(i.request.clone(), req.clone());
            (i.clone(), mismatches)
        }).collect();
        match matches.iter().find(|i| i.1.is_empty()) {
            Some(i) => MatchResult::RequestMatch(i.clone().0.response),
            None => MatchResult::RequestMismatch(matches.clone())
        }
    }
}

fn extract_path(uri: &RequestUri) -> String {
    match uri {
        &RequestUri::AbsolutePath(ref s) => s!(s.splitn(2, "?").next().unwrap_or("/")),
        &RequestUri::AbsoluteUri(ref url) => url.path().unwrap_or(&[s!("")]).join("/"),
        _ => uri.to_string()
    }
}

fn extract_query_string(uri: &RequestUri) -> Option<HashMap<String, Vec<String>>> {
    match uri {
        &RequestUri::AbsolutePath(ref s) => match s.splitn(2, "?").last() {
            Some(q) => parse_query_string(&s!(q)),
            None => None
        },
        &RequestUri::AbsoluteUri(ref url) => match url.query {
            Some(ref q) => parse_query_string(q),
            None => None
        },
        _ => None
    }
}

fn extract_headers(headers: &Headers) -> Option<HashMap<String, String>> {
    if headers.len() > 0 {
        Some(headers.iter().map(|h| (s!(h.name()), h.value_string()) ).collect())
    } else {
        None
    }
}

fn extract_body(req: &mut hyper::server::Request) -> OptionalBody {
    let mut buffer = String::new();
    match req.read_to_string(&mut buffer) {
        Ok(size) => if size > 0 {
                OptionalBody::Present(buffer)
            } else {
                OptionalBody::Empty
            },
        Err(err) => {
            warn!("Failed to read request body: {}", err);
            OptionalBody::Empty
        }
    }
}

fn hyper_request_to_pact_request(req: &mut hyper::server::Request) -> Request {
    Request {
        method: req.method.to_string(),
        path: extract_path(&req.uri),
        query: extract_query_string(&req.uri),
        headers: extract_headers(&req.headers),
        body: extract_body(req),
        matching_rules: None
    }
}

fn error_body(req: &Request) -> String {
    let body = hashmap!{ "error" => format!("Unexpected request : {:?}", req) };
    let json = json::encode(&body).unwrap();
    json.clone()
}

fn start_mock_server(pact: Pact) -> Result<i32, String> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let server_result = Server::http("0.0.0.0:0").unwrap().handle(move |mut req: hyper::server::Request, mut res: hyper::server::Response| {
            let req = hyper_request_to_pact_request(&mut req);
            debug!("Received request {:?}", req);
            match match_request(&req, &pact.interactions) {
                MatchResult::RequestMatch(ref response) => {
                    debug!("Request matched, sending response {:?}", response);
                    *res.status_mut() = StatusCode::from_u16(response.status);
                    match response.headers {
                        Some(ref headers) => {
                            for (k, v) in headers.clone() {
                                res.headers_mut().set_raw(k, vec![v.into_bytes()]);
                            }
                        },
                        None => ()
                    }
                    match response.body {
                        OptionalBody::Present(ref body) => {
                            res.send(body.as_bytes()).unwrap();
                        },
                        _ => ()
                    }
                },
                _ => {
                    *res.status_mut() = StatusCode::InternalServerError;
                    res.headers_mut().set(
                        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                         vec![(Attr::Charset, Value::Utf8)]))
                    );
                    res.headers_mut().set(AccessControlAllowOrigin::Any);
                    res.headers_mut().set_raw("X-Pact", vec![b"Unexpected-Request".to_vec()]);
                    let body = error_body(&req);
                    res.headers_mut().set(ContentLength(body.as_bytes().len() as u64));
                    let mut res = res.start().unwrap();
                    res.write_all(body.as_bytes()).unwrap();
                }
            }
        });

        match server_result {
            Ok(server) => {
                info!("Mock Provider Server started on port {}", server.socket.port());
                tx.send((server.socket.port() as i32, s!(""))).unwrap();
            },
            Err(e) => {
                error!("Could not start server: {}", e);
                tx.send((-1, format!("Could not start server: {}", e))).unwrap();
            }
        }
    });

    match rx.recv().unwrap() {
        (-1, err) => Err(err),
        (port, _) => {
            let mock_server = MockServer { port: port };
            match MOCK_SERVERS.write() {
                Ok(mut map) => {
                    map.insert(port, mock_server);
                },
                Err(poisoned) => {
                    let mut map = poisoned.into_inner();
                    map.insert(port, mock_server);
                }
            }
            Ok(port)
        }
    }
}

#[no_mangle]
pub extern fn create_mock_server(pact_str: *const c_char) -> int32_t {
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
            let pact = Pact::from_json(&pact_json);
            p!(pact);
            match start_mock_server(pact) {
                Ok(mock_server) => mock_server as i32,
                Err(msg) => {
                    error!("Could not start mock server: {}", msg);
                    -3
                }
            }
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

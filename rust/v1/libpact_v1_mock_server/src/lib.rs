#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;
extern crate libc;
#[macro_use] extern crate libpact_v1_matching;
extern crate rustc_serialize;
extern crate env_logger;
#[macro_use] extern crate hyper;
extern crate uuid;

use libc::{c_char, int32_t};
use std::ffi::CStr;
use std::str;
use libpact_v1_matching::models::{Pact, Interaction, Request, Response, OptionalBody};
use libpact_v1_matching::models::parse_query_string;
use libpact_v1_matching::Mismatch;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::collections::{BTreeMap, HashMap};
use std::cell::Cell;
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::io::{Read, Write};
use hyper::server::Server;
use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType, AccessControlAllowOrigin, ContentLength};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::uri::RequestUri;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    RequestMatch(Response),
    RequestMismatch(Vec<(Interaction, Vec<Mismatch>)>),
    RequestNotFound(Request)
}

impl MatchResult {
    pub fn match_key(&self) -> String {
        match self {
            &MatchResult::RequestMatch(_) => s!("Request-Matched"),
            &MatchResult::RequestMismatch(_) => s!("Request-Mismatch"),
            &MatchResult::RequestNotFound(_) => s!("Unexpected-Request")
        }
    }

    pub fn matched(&self) -> bool {
        match self {
            &MatchResult::RequestMatch(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockServer {
    pub id: String,
    pub port: i32,
    pub matches: Vec<MatchResult>
}

impl MockServer {
    pub fn new(id: String) -> MockServer {
        MockServer { id: id.clone(), port: -1, matches: vec![] }
    }

    pub fn port(&mut self, port: i32) {
        self.port = port;
    }
}

lazy_static! {
    static ref MOCK_SERVERS: Mutex<BTreeMap<String, Box<MockServer>>> = Mutex::new(BTreeMap::new());
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
        &RequestUri::AbsolutePath(ref s) => {
            if s.contains("?") {
                match s.splitn(2, "?").last() {
                    Some(q) => parse_query_string(&s!(q)),
                    None => None
                }
            } else {
                None
            }
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

fn error_body(req: &Request, error: &String) -> String {
    let body = hashmap!{ "error" => format!("{} : {:?}", error, req) };
    let json = json::encode(&body).unwrap();
    json.clone()
}

fn insert_new_mock_server(id: &String) {
    MOCK_SERVERS.lock().unwrap().insert(id.clone(), Box::new(MockServer::new(id.clone())));
}

fn update_mock_server(id: &String, f: &Fn(&mut MockServer) -> ()) -> bool {
    match MOCK_SERVERS.lock().unwrap().get_mut(id) {
        Some(mock_server) => { f(mock_server); true },
        _ => false
    }
}

fn record_result(id: &String, match_result: &MatchResult) {
    update_mock_server(id, &|mock_server: &mut MockServer| {
        mock_server.matches.push(match_result.clone());
    });
}

fn start_mock_server(id: String, pact: Pact) -> Result<i32, String> {
    insert_new_mock_server(&id);
    let (out_tx, out_rx) = channel();
    let (in_tx, in_rx) = channel();
    in_tx.send((id.clone(), pact)).unwrap();
    thread::spawn(move || {
        let (mock_server_id, pact) = in_rx.recv().unwrap();
        let server = Server::http("0.0.0.0:0").unwrap();
        let server_result = server.handle(move |mut req: hyper::server::Request, mut res: hyper::server::Response| {
            let req = hyper_request_to_pact_request(&mut req);
            debug!("Received request {:?}", req);
            let match_result = match_request(&req, &pact.interactions);
            record_result(&mock_server_id, &match_result);
            match match_result {
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
                    res.headers_mut().set_raw("X-Pact", vec![match_result.match_key().as_bytes().to_vec()]);
                    let body = error_body(&req, &match_result.match_key());
                    res.headers_mut().set(ContentLength(body.as_bytes().len() as u64));
                    let mut res = res.start().unwrap();
                    res.write_all(body.as_bytes()).unwrap();
                }
            }
        });

        match server_result {
            Ok(server) => {
                let port = server.socket.port() as i32;
                info!("Mock Provider Server started on port {}", port);
                out_tx.send(Ok(port)).unwrap();
            },
            Err(e) => {
                error!("Could not start server: {}", e);
                out_tx.send(Err(format!("Could not start server: {}", e))).unwrap();
            }
        }
    });

    match out_rx.recv().unwrap() {
        Err(err) => Err(err),
        Ok(port) => {
            update_mock_server(&id, &|mock_server| mock_server.port(port) );
            Ok(port)
        }
    }
}

fn lookup_mock_server(mock_server_port: i32, f: &Fn(&mut MockServer)) -> bool {
    let mut map = MOCK_SERVERS.lock().unwrap();
    match map.iter_mut().find(|ms| ms.1.port == mock_server_port ) {
        Some(mock_server) => { f(&mut *mock_server.1); true },
        None => false
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
            match start_mock_server(Uuid::new_v4().to_string(), pact) {
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

#[no_mangle]
pub extern fn mock_server_matched(mock_server_port: int32_t) -> bool {
    let ok = Cell::new(false);
    if lookup_mock_server(mock_server_port, &|mock_server| {
        ok.set(!mock_server.matches.iter().any(|mismatch| !mismatch.matched()));
    }) {
        ok.get()
    } else {
        false
    }
}

#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;
extern crate libc;
#[macro_use] extern crate pact_matching;
extern crate rustc_serialize;
extern crate env_logger;
#[macro_use] extern crate hyper;
extern crate uuid;
#[macro_use] extern crate itertools;

use libc::{c_char, int32_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use pact_matching::models::{Pact, Interaction, Request, OptionalBody};
use pact_matching::models::parse_query_string;
use pact_matching::Mismatch;
use rustc_serialize::json::{self, Json, ToJson};
use std::collections::{BTreeMap, HashMap};
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::io::{Read, Write};
use hyper::server::{Server, Listening};
use hyper::status::StatusCode;
use hyper::header::{Headers, ContentType, AccessControlAllowOrigin, ContentLength};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::uri::RequestUri;
use uuid::Uuid;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    RequestMatch(Interaction),
    RequestMismatch(Interaction, Vec<Mismatch>),
    RequestNotFound(Request),
    MissingRequest(Interaction)
}

impl MatchResult {
    pub fn match_key(&self) -> String {
        match self {
            &MatchResult::RequestMatch(_) => s!("Request-Matched"),
            &MatchResult::RequestMismatch(_, _) => s!("Request-Mismatch"),
            &MatchResult::RequestNotFound(_) => s!("Unexpected-Request"),
            &MatchResult::MissingRequest(_) => s!("Missing-Request")
        }
    }

    pub fn matched(&self) -> bool {
        match self {
            &MatchResult::RequestMatch(_) => true,
            _ => false
        }
    }

    pub fn to_json(&self) -> Json {
        match self {
            &MatchResult::RequestMatch(_) => Json::Object(btreemap!{ s!("type") => s!("request-match").to_json() }),
            &MatchResult::RequestMismatch(ref interaction, ref mismatches) => mismatches_to_json(&interaction.request, mismatches),
            &MatchResult::RequestNotFound(ref req) => Json::Object(btreemap!{
                s!("type") => s!("request-not-found").to_json(),
                s!("method") => req.method.to_json(),
                s!("path") => req.path.to_json(),
                s!("request") => req.to_json()
            }),
            &MatchResult::MissingRequest(ref interaction) => Json::Object(btreemap!{
                s!("type") => s!("missing-request").to_json(),
                s!("method") => interaction.request.method.to_json(),
                s!("path") => interaction.request.path.to_json(),
                s!("request") => interaction.request.to_json()
            })
        }
    }
}

fn mismatches_to_json(request: &Request, mismatches: &Vec<Mismatch>) -> Json {
    Json::Object(btreemap!{
        s!("type") => s!("request-mismatch").to_json(),
        s!("method") => request.method.to_json(),
        s!("path") => request.path.to_json(),
        s!("mismatches") => Json::Array(mismatches.iter().map(|m| m.to_json()).collect())
    })
}

pub struct MockServer {
    pub id: String,
    pub port: i32,
    pub server: u64,
    pub matches: Vec<MatchResult>,
    pub resources: Vec<CString>,
    pub pact: Pact
}

impl MockServer {
    pub fn new(id: String, pact: &Pact) -> MockServer {
        MockServer { id: id.clone(), port: -1, server: 0, matches: vec![], resources: vec![],
            pact : pact.clone() }
    }

    pub fn port(&mut self, port: i32) {
        self.port = port;
    }

    pub fn server(&mut self, server: &Listening) {
        let p = server as *const Listening;
        self.server = p as u64;
    }

    pub fn to_json(&self) -> Json {
        Json::Object(btreemap!{
            s!("id") => Json::String(self.id.clone()),
            s!("port") => Json::U64(self.port as u64),
            s!("provider") => Json::String(self.pact.provider.name.clone()),
            s!("status") => Json::String(if self.mismatches().is_empty() {
                    s!("ok")
                } else {
                    s!("error")
                }
            )
        })
    }

    pub fn mismatches(&self) -> Vec<MatchResult> {
        let mismatches = self.matches.iter()
            .filter(|m| !m.matched())
            .map(|m| m.clone());
        let interactions: Vec<&Interaction> = self.matches.iter().map(|m| {
            match *m {
                MatchResult::RequestMatch(ref interaction) => Some(interaction),
                MatchResult::RequestMismatch(ref interaction, _) => Some(interaction),
                MatchResult::RequestNotFound(_) => None,
                MatchResult::MissingRequest(_) => None
            }
        }).filter(|o| o.is_some()).map(|o| o.unwrap()).collect();
        let missing = self.pact.interactions.iter()
            .filter(|i| !interactions.contains(i))
            .map(|i| MatchResult::MissingRequest(i.clone()));
        mismatches.chain(missing).collect()
    }
}

impl PartialEq for MockServer {
    fn eq(&self, other: &MockServer) -> bool {
        self.id == other.id
    }
}

lazy_static! {
    static ref MOCK_SERVERS: Mutex<BTreeMap<String, Box<MockServer>>> = Mutex::new(BTreeMap::new());
}

fn match_request(req: &Request, interactions: &Vec<Interaction>) -> MatchResult {
    let match_results = interactions
        .into_iter()
        .map(|i| (i.clone(), pact_matching::match_request(i.request.clone(), req.clone())))
        .sorted_by(|i1, i2| {
            let list1 = i1.1.clone().into_iter().map(|m| m.mismatch_type()).unique().count();
            let list2 = i2.1.clone().into_iter().map(|m| m.mismatch_type()).unique().count();
            Ord::cmp(&list1, &list2)
        });
    match match_results.first() {
        Some(res) => {
            if res.1.is_empty() {
                MatchResult::RequestMatch(res.0.clone())
            } else if method_or_path_mismatch(&res.1) {
                MatchResult::RequestNotFound(req.clone())
            } else {
                MatchResult::RequestMismatch(res.0.clone(), res.1.clone())
            }
        },
        None => MatchResult::RequestNotFound(req.clone())
    }
}

fn method_or_path_mismatch(mismatches: &Vec<Mismatch>) -> bool {
    let mismatch_types: Vec<String> = mismatches.iter()
        .map(|mismatch| mismatch.mismatch_type())
        .collect();
    mismatch_types.contains(&s!("MethodMismatch")) || mismatch_types.contains(&s!("PathMismatch"))
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

fn insert_new_mock_server(id: &String, pact: &Pact) {
    MOCK_SERVERS.lock().unwrap().insert(id.clone(), Box::new(MockServer::new(id.clone(), pact)));
}

fn update_mock_server<R>(id: &String, f: &Fn(&mut MockServer) -> R) -> Option<R> {
    match MOCK_SERVERS.lock().unwrap().get_mut(id) {
        Some(mock_server) => Some(f(mock_server)),
        _ => None
    }
}

fn update_mock_server_by_port<R>(port: i32, f: &Fn(&mut MockServer) -> R) -> Option<R> {
    let mut map = MOCK_SERVERS.lock().unwrap();
    match map.iter_mut().find(|ms| ms.1.port == port ) {
        Some(mock_server) => Some(f(mock_server.1)),
        None => None
    }
}

fn record_result(id: &String, match_result: &MatchResult) {
    update_mock_server(id, &|mock_server: &mut MockServer| {
        mock_server.matches.push(match_result.clone());
    });
}

pub fn start_mock_server(id: String, pact: Pact) -> Result<i32, String> {
    insert_new_mock_server(&id, &pact);
    let (out_tx, out_rx) = channel();
    let (in_tx, in_rx) = channel();
    in_tx.send((id.clone(), pact)).unwrap();
    thread::spawn(move || {
        let (mock_server_id, pact) = in_rx.recv().unwrap();
        let server = Server::http("0.0.0.0:0").unwrap();
        let server_result = server.handle(move |mut req: hyper::server::Request, mut res: hyper::server::Response| {
            let req = hyper_request_to_pact_request(&mut req);
            info!("Received request {:?}", req);
            let match_result = match_request(&req, &pact.interactions);
            record_result(&mock_server_id, &match_result);
            match match_result {
                MatchResult::RequestMatch(ref interaction) => {
                    info!("Request matched, sending response {:?}", interaction.response);
                    *res.status_mut() = StatusCode::from_u16(interaction.response.status);
                    res.headers_mut().set(AccessControlAllowOrigin::Any);
                    match interaction.response.headers {
                        Some(ref headers) => {
                            for (k, v) in headers.clone() {
                                res.headers_mut().set_raw(k, vec![v.into_bytes()]);
                            }
                        },
                        None => ()
                    }
                    match interaction.response.body {
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
            Ok(ref server) => {
                let port = server.socket.port() as i32;
                info!("Mock Provider Server started on port {}", port);
                update_mock_server(&id, &|mock_server| {
                    mock_server.port(port);
                    mock_server.server(server);
                });
                out_tx.send(Ok(port)).unwrap();
            },
            Err(e) => {
                error!("Could not start server: {}", e);
                out_tx.send(Err(format!("Could not start server: {}", e))).unwrap();
            }
        }
    });

    out_rx.recv().unwrap()
}

pub fn lookup_mock_server<R>(id: String, f: &Fn(&MockServer) -> R) -> Option<R> {
    let map = MOCK_SERVERS.lock().unwrap();
    match map.get(&id) {
        Some(ref mock_server) => Some(f(mock_server)),
        None => None
    }
}

pub fn lookup_mock_server_by_port<R>(mock_server_port: i32, f: &Fn(&MockServer) -> R) -> Option<R> {
    let map = MOCK_SERVERS.lock().unwrap();
    match map.iter().find(|ms| ms.1.port == mock_server_port ) {
        Some(mock_server) => Some(f(mock_server.1)),
        None => None
    }
}

pub fn iterate_mock_servers(f: &mut FnMut(&String, &MockServer)) {
    let map = MOCK_SERVERS.lock().unwrap();
    for (key, value) in map.iter() {
        f(key, value);
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
            match start_mock_server(Uuid::new_v4().simple().to_string(), pact) {
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
    lookup_mock_server_by_port(mock_server_port, &|mock_server| {
        mock_server.mismatches().is_empty()
    }).unwrap_or(false)
}

#[no_mangle]
pub extern fn mock_server_mismatches(mock_server_port: int32_t) -> *mut c_char {
    let result = update_mock_server_by_port(mock_server_port, &|ref mut mock_server| {
        let mismatches = mock_server.mismatches().iter()
            .map(|mismatch| mismatch.to_json() )
            .collect::<Vec<Json>>();
        let json = Json::Array(mismatches);
        let s = CString::new(json.to_string()).unwrap();
        let p = s.as_ptr();
        mock_server.resources.push(s);
        p
    });
    match result {
        Some(p) => p as *mut _,
        None => 0 as *mut _
    }
}

#[no_mangle]
pub extern fn cleanup_mock_server(mock_server_port: int32_t) -> bool {
    let id_result = update_mock_server_by_port(mock_server_port, &|mock_server| {
        mock_server.resources.clear();
        if mock_server.server > 0 {
            let server_raw = mock_server.server as *mut Listening;
            let mut server_ref = unsafe { &mut *server_raw };
            server_ref.close().unwrap();
        }
        mock_server.id.clone()
    });

    match id_result {
        Some(ref id) => {
            MOCK_SERVERS.lock().unwrap().remove(id);
            true
        },
        None => false
    }
}

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests;

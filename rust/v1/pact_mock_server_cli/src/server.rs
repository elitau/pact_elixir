use hyper::server::{Server, Request, Response};
use hyper::method::Method;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use pact_matching::models::Pact;
use pact_mock_server::{
    start_mock_server,
    iterate_mock_servers,
    MockServer
};
use uuid::Uuid;
use std::error::Error;
use rustc_serialize::json::Json;
use std::borrow::Borrow;
use std::iter::FromIterator;
use verify;
use clap::ArgMatches;
use webmachine_rust::{WebmachineDispatcher, WebmachineResource};
use regex::Regex;

// impl Handler for MasterServerHandler {
//     fn handle_request(&self, context: Context, response: Response) {
//         match context.method {
//             Get => list_servers(response),
//             Post => {
//                 let path = context.uri.clone();
//                 if path.as_utf8_path().unwrap() == "/" {
//                     start_provider(context, response)
//                 } else {
//                     verify_mock_server_request(context, response, &self.output_path)
//                 }
//             },
//             _ => ()
//         }
//     }
// }

// fn start_provider(mut context: Context, mut response: Response) {
//     add_cors_headers(&mut response);
//     let json_result = context.body.read_json_body();
//     match json_result {
//         Ok(pact_json) => {
//             let pact = Pact::from_json(&pact_json);
//             let mock_server_id = Uuid::new_v4().simple().to_string();
//             match start_mock_server(mock_server_id.clone(), pact, 0) {
//                 Ok(mock_server) => {
//                     response.set_status(StatusCode::Ok);
//                     response.headers_mut().set(
//                         ContentType(Mime(TopLevel::Application, SubLevel::Json,
//                                          vec![(Attr::Charset, Value::Utf8)]))
//                     );
//                     let mock_server_json = Json::Object(btreemap!{
//                         s!("id") => Json::String(mock_server_id),
//                         s!("port") => Json::I64(mock_server as i64),
//                     });
//                     let json_response = Json::Object(btreemap!{ s!("mockServer") => mock_server_json });
//                     response.send(json_response.to_string());
//                 },
//                 Err(msg) => {
//                     response.set_status(StatusCode::BadRequest);
//                     response.send(msg);
//                 }
//             }
//         },
//         Err(err) => {
//             error!("Could not parse pact json: {}", err);
//             response.set_status(StatusCode::BadRequest);
//             response.send(err.description());
//         }
//     }
// }
//
// fn list_servers(mut response: Response) {
//     add_cors_headers(&mut response);
//     response.set_status(StatusCode::Ok);
//     response.headers_mut().set(
//         ContentType(Mime(TopLevel::Application, SubLevel::Json,
//                          vec![(Attr::Charset, Value::Utf8)]))
//     );
//

//
//     let json_response = Json::Object(btreemap!{ s!("mockServers") => Json::Array(mock_servers) });
//     response.send(json_response.to_string());
// }
//
// pub fn verify_mock_server_request(context: Context, mut response: Response, output_path: &Option<String>) {
//     add_cors_headers(&mut response);
//     response.headers_mut().set(
//         ContentType(Mime(TopLevel::Application, SubLevel::Json,
//                          vec![(Attr::Charset, Value::Utf8)]))
//     );
//
//     let id = context.variables.get("id").unwrap();
//     match verify::validate_id(id.borrow()) {
//         Ok(ms) => {
//             let mut map = btreemap!{ s!("mockServer") => ms.to_json() };
//             let mismatches = ms.mismatches();
//             if !mismatches.is_empty() {
//                 response.set_status(StatusCode::BadRequest);
//                 map.insert(s!("mismatches"), Json::Array(
//                     Vec::from_iter(mismatches.iter()
//                         .map(|m| m.to_json()))));
//             } else {
//                 match ms.write_pact(output_path) {
//                     Ok(_) => response.set_status(StatusCode::Ok),
//                     Err(err) => {
//                         response.set_status(StatusCode::UnprocessableEntity);
//                         map.insert(s!("error"), Json::String(format!("Failed to write pact to file - {}", err)));
//                     }
//                 }
//             }
//
//             let json_response = Json::Object(map);
//             response.send(json_response.to_string());
//         },
//         Err(err) => {
//             response.set_status(StatusCode::UnprocessableEntity);
//             response.send(Json::String(err).to_string());
//         }
//     }
// }

pub fn start_server(port: u16, matches: &ArgMatches) -> Result<(), i32> {
    let output_path = matches.value_of("output").map(|o| s!(o));

    match Server::http(format!("0.0.0.0:{}", port).as_str()) {
        Ok(mut server) => {
            server.keep_alive(None);
            match server.handle(move |req: Request, res: Response| {
                let main_resource = WebmachineResource {
                    allowed_methods: vec![s!("OPTIONS"), s!("GET"), s!("HEAD"), s!("POST")],
                    resource_exists: Box::new(|context| context.request.request_path == "/"),
                    render_response: Box::new(|_| {
                        let mut mock_servers = vec![];
                        iterate_mock_servers(&mut |_: &String, ms: &MockServer| {
                            let mock_server_json = ms.to_json();
                            mock_servers.push(mock_server_json);
                        });
                        let json_response = Json::Object(btreemap!{ s!("mockServers") => Json::Array(mock_servers) });
                        Some(json_response.to_string())
                    }),
                    .. WebmachineResource::default()
                };
                let mock_server_resource = WebmachineResource {
                    allowed_methods: vec![s!("OPTIONS"), s!("GET"), s!("HEAD"), s!("POST")],
                    resource_exists: Box::new(|context| {
                        p!(context.request);
                        let re = Regex::new(r"^/\d+").unwrap();
                        re.is_match(&context.request.request_path)
                    }),
                    .. WebmachineResource::default()
                };

                let dispatcher = WebmachineDispatcher {
                    routes: btreemap!{
                        s!("/") => main_resource,
                        s!("/mockserver") => mock_server_resource
                    }
                };

                match dispatcher.dispatch(req, res) {
                    Ok(_) => (),
                    Err(err) => warn!("Error generating response - {}", err)
                };
            }) {
                Ok(listener) => {
                    info!("Server started on port {}", listener.socket.port());
                    Ok(())
                },
                Err(err) => {
                    error!("could not bind listener to port: {}", err);
                    Err(2)
                }
            }
        },
        Err(err) => {
            error!("could not start server: {}", err);
            Err(1)
        }
    }
}

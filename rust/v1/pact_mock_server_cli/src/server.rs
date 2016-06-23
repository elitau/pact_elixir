use hyper::server::{Server, Request, Response};
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
use webmachine_rust::*;
use webmachine_rust::context::*;
use webmachine_rust::headers::*;
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

fn json_error(error: String) -> String {
    let json_response = Json::Object(btreemap!{ s!("error") => Json::String(error) });
    json_response.to_string()
}

fn start_provider(context: &mut WebmachineContext) -> Result<bool, u16> {
    match context.request.body {
        Some(ref body) if !body.is_empty() => {
            match Json::from_str(body) {
                Ok(ref json) => {
                    let pact = Pact::from_json(json);
                    let mock_server_id = Uuid::new_v4().simple().to_string();
                    match start_mock_server(mock_server_id.clone(), pact, 0) {
                        Ok(mock_server) => {
                            let mock_server_json = Json::Object(btreemap!{
                                s!("id") => Json::String(mock_server_id.clone()),
                                s!("port") => Json::I64(mock_server as i64),
                            });
                            let json_response = Json::Object(btreemap!{ s!("mockServer") => mock_server_json });
                            context.response.body = Some(json_response.to_string());
                            context.response.add_header(s!("Location"),
                                vec![HeaderValue::basic(&format!("/mockserver/{}", mock_server_id))]);
                            Ok(true)
                        },
                        Err(msg) => {
                            context.response.body = Some(json_error(format!("Failed to start mock server - {}", msg)));
                            Err(422)
                        }
                    }
                },
                Err(err) => {
                    error!("Failed to parse json body - {}", err);
                    context.response.body = Some(json_error(format!("Failed to parse json body - {}", err)));
                    Err(422)
                }
            }
        },
        _ => {
            error!("No pact json was supplied");
            context.response.body = Some(json_error(s!("No pact json was supplied")));
            Err(422)
        }
    }
}

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
                    process_post: Box::new(|context| start_provider(context)),
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

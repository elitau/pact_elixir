use rustful::{
    Server,
    Handler,
    Context,
    Response,
    TreeRouter
};
use rustful::StatusCode;
use rustful::header::{
    ContentType,
    AccessControlAllowOrigin,
    AccessControlAllowMethods,
    AccessControlAllowHeaders,
    Host
};
use hyper::method::Method;
use pact_v1_matching::models::Pact;
use pact_v1_mock_server::start_mock_server;
use uuid::Uuid;
use std::error::Error;
use rustc_serialize::json::Json;

fn add_cors_headers(response: &mut Response) {
    response.headers_mut().set(AccessControlAllowOrigin::Any);
    response.headers_mut().set(AccessControlAllowMethods(vec![Method::Post]));
    response.headers_mut().set(AccessControlAllowHeaders(vec!["Content-Type".into()]));
}

fn start_provider(mut context: Context, mut response: Response) {
    add_cors_headers(&mut response);
    let json_result = context.body.read_json_body();
    match json_result {
        Ok(pact_json) => {
            let pact = Pact::from_json(&pact_json);
            let mock_server_id = Uuid::new_v4().to_string();
            match start_mock_server(mock_server_id.clone(), pact) {
                Ok(mock_server) => {
                    response.set_status(StatusCode::Ok);
                    let mock_server_json = Json::Object(btreemap!{
                        s!("id") => Json::String(mock_server_id),
                        s!("port") => Json::I64(mock_server as i64),
                    });
                    let json_response = Json::Object(btreemap!{ s!("mockServer") => mock_server_json });
                    response.send(json_response.to_string());
                },
                Err(msg) => {
                    response.set_status(StatusCode::BadRequest);
                    response.send(msg);
                }
            }
        },
        Err(err) => {
            error!("Could not parse pact json: {}", err);
            response.set_status(StatusCode::BadRequest);
            response.send(err.description());
        }
    }
}

pub fn start_command() {
    let my_router = insert_routes!{
        TreeRouter::new() => {
            Post: start_provider
        }
    };

    let server_result = Server {
        handlers: my_router,
        host: 8080.into(),
        ..Server::default()
    }.run();

    match server_result {
        Ok(server) => info!("Server started on port {}", server.socket.port()),
        Err(e) => panic!("could not start server: {}", e)
    }
}

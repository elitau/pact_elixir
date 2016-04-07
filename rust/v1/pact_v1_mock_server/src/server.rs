use rustful::{Server, Handler, Context, Response, TreeRouter};
use std::thread;
use std::sync::mpsc::channel;
use rustful::StatusCode;
use pact_v1_matching::models::Pact;
use pact_v1_mock_server::start_mock_server;
use uuid::Uuid;
use rustc_serialize::json::ParserError;
use  std::error::Error;

fn start_provider(mut context: Context, mut response: Response) {
    let json_result = context.body.read_json_body();
    match json_result {
        Ok(pact_json) => {
            let pact = Pact::from_json(&pact_json);
            match start_mock_server(Uuid::new_v4().to_string(), pact) {
                Ok(mock_server) => {

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
            // Get: list_root,
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

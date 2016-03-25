

use rustful::{Server, Handler, Context, Response, TreeRouter};
use std::thread;
use std::sync::mpsc::channel;
use rustful::StatusCode;

fn list_root(context: Context, mut response: Response) {
    response.send("Hello World!");
}

fn start_provider(context: Context, mut response: Response) {
    let my_router = insert_routes!{
        TreeRouter::new() => {
            Get: list_root
        }
    };

    let (tx, rx) = channel();
    let child = thread::spawn(move || {
        let server_result = Server {
            handlers: my_router,
            host: 8081.into(),
            ..Server::default()
        }.run();

        match server_result {
            Ok(server) => {
                info!("Provider Server started on port {}", server.socket.port());
                tx.send("OK".to_string()).unwrap();
                tx.send(format!("Provider Server started on port {}", server.socket.port())).unwrap();
            },
            Err(e) => {
                error!("could not start server: {}", e);
                tx.send("ERROR".to_string()).unwrap();
                tx.send(format!("could not start server: {}", e));
            }
        }
    });
    match &*rx.recv().unwrap() {
        "OK" => response.set_status(StatusCode::Ok),
        _ => response.set_status(StatusCode::BadRequest)
    }
    response.send(rx.recv().unwrap());
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

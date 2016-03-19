

use rustful::{Server, Handler, Context, Response, TreeRouter};

fn list_root(context: Context, mut response: Response) {
    response.send("Hello World!");
}

pub fn start_command() {
    let my_router = insert_routes!{
        TreeRouter::new() => {
            // //Receive GET requests to /hello and /hello/:name
            // "hello" => {
            //     Get: Greeting("hello"),
            //     ":name" => Get: Greeting("hello")
            // },
            // //Receive GET requests to /good_bye and /good_bye/:name
            // "good_bye" => {
            //     Get: Greeting("good bye"),
            //     ":name" => Get: Greeting("good bye")
            // }

            Get: list_root
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

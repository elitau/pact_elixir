use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use hyper::status::*;

pub fn shutdown_mock_server(host: &str, port: u16, matches: &ArgMatches) -> Result<(), i32> {
    let mock_server_id = matches.value_of("mock-server-id");
    let mock_server_port = matches.value_of("mock-server-port");
    let id = if mock_server_id.is_some() {
        (mock_server_id.unwrap(), "id")
    } else {
        (mock_server_port.unwrap(), "port")
    };

    let client = Client::new();
    let url = Url::parse(format!("http://{}:{}/mockserver/{}", host, port, id.0)
        .as_str()).unwrap();
    let res = client.delete(url.clone()).send();

    match res {
        Ok(result) => {
            if !result.status.is_success() {
                match result.status {
                    StatusCode::NotFound => {
                        println!("No mock server found with {} '{}', use the 'list' command to get a list of available mock servers.",
                            id.1, id.0);
                        Err(3)
                    },
                    _ => ::display_error(format!("Unexpected response from master mock server '{}': {}", url, result.status), matches)
                }
            } else {
                println!("Mock server with {} '{}' shutdown ok", id.1, id.0);
                Ok(())
            }
        },
        Err(err) => {
            ::display_error(format!("Failed to connect to the master mock server '{}': {}", url, err), matches);
        }
    }
}

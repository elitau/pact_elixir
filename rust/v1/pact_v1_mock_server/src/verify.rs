use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use std::io::prelude::*;
use rustc_serialize::json::Json;

pub fn verify_mock_server(host: &str, port: u16, matches: &ArgMatches) {
    let mock_server_id = matches.value_of("mock-server-id");
    let mock_server_port = matches.value_of("mock-server-port");

    let client = Client::new();
    let url = Url::parse(format!("http://{}:{}/mockserver/{}/verify", host, port,
        if mock_server_id.is_some() { mock_server_id.unwrap() } else { mock_server_port.unwrap() })
        .as_str()).unwrap();
    let res = client.post(url.clone()).send();

    match res {
        Ok(mut result) => {
            let mut body = String::new();
            result.read_to_string(&mut body).unwrap();
            if result.status.is_success() {
                let json_result = Json::from_str(body.as_str());
                match json_result {
                    Ok(json) => {
                        let mock_server = json.find("mockServer").unwrap();
                        let id = mock_server.find("id").unwrap();
                        let port = mock_server.find("port").unwrap();
                        let status = mock_server.find("status").unwrap().as_boolean().unwrap();
                        if status {
                            println!("Mock server {}/{} verified ok", id, port);
                        } else {
                            println!("Mock server {}/{} failed verification", id, port);
                        }
                    },
                    Err(_) => {
                        println!("Failed to parse JSON: {}", body);
                    }
                }
            } else {
                ::display_error(format!("Master mock server returned an error: {}\n{}", result.status, body), matches);
            }
        },
        Err(err) => {
            ::display_error(format!("Failed to connect to the master mock server '{}': {}", url, err), matches);
        }
    }
}

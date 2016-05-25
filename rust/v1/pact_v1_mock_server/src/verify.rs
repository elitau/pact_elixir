use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use std::io::prelude::*;
use rustc_serialize::json::Json;
use pact_v1_mock_server::{
    lookup_mock_server,
    lookup_mock_server_by_port,
    MockServer
};

pub fn verify_mock_server(host: &str, port: u16, matches: &ArgMatches) -> Result<(), i32> {
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
            let json_result = Json::from_str(body.as_str());
            match json_result {
                Ok(json) => {
                    let mock_server = json.find("mockServer").unwrap();
                    let id = mock_server.find("id").unwrap().as_string().unwrap();
                    let port = mock_server.find("port").unwrap();
                    let status = mock_server.find("status").unwrap().as_string().unwrap();
                    if status == "ok" {
                        println!("Mock server {}/{} verified ok", id, port);
                        Ok(())
                    } else {
                        println!("Mock server {}/{} failed verification", id, port);
                        Err(2)
                    }
                },
                Err(err) => {
                    error!("Failed to parse JSON: {}\n{}", err, body);
                    ::display_error(format!("Failed to parse JSON: {}\n{}", err, body), matches);
                }
            }
        },
        Err(err) => {
            ::display_error(format!("Failed to connect to the master mock server '{}': {}", url, err), matches);
        }
    }
}

fn validate_port(id: u16) -> Result<MockServer, String> {
    lookup_mock_server_by_port(id as i32, &|ref ms| {
        MockServer {
            id: ms.id.clone(),
            port: ms.port,
            server: ms.server,
            matches: ms.matches.clone(),
            resources: vec![],
            pact: ms.pact.clone()
        }
    }).ok_or(format!("No mock server running with port '{}'", id))
}

fn validate_uuid(id: &String) -> Result<MockServer, String> {
    lookup_mock_server(id.clone(), &|ref ms| {
        MockServer {
            id: ms.id.clone(),
            port: ms.port,
            server: ms.server,
            matches: ms.matches.clone(),
            resources: vec![],
            pact: ms.pact.clone()
        }
    }).ok_or(format!("No mock server running with id '{}'", id))
}

pub fn validate_id(id: &str) -> Result<MockServer, String> {
    if id.chars().all(|ch| ch.is_digit(10)) {
        validate_port(id.parse::<u16>().unwrap())
    } else {
        validate_uuid(&s!(id))
    }
}

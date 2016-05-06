use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use std::io::prelude::*;
use rustc_serialize::json::Json;

pub fn list_mock_servers(host: &str, port: u16, matches: &ArgMatches) {
    let client = Client::new();
    let url = Url::parse(format!("http://{}:{}/", host, port).as_str()).unwrap();
    let res = client.get(url.clone())
        .send();

    match res {
        Ok(mut result) => {
            let mut body = String::new();
            result.read_to_string(&mut body).unwrap();
            if result.status.is_success() {
                let json_result = Json::from_str(body.as_str());
                match json_result {
                    Ok(json) => {
                        let mock_servers_json = json.find("mockServers").unwrap();
                        let mock_servers = mock_servers_json.as_array().unwrap();

                        println!("Mock Server Id                     \tPort");
                        for ms in mock_servers {
                            let id = ms.find("id").unwrap().as_string().unwrap();
                            let port = ms.find("port").unwrap();
                            println!("{}\t{}", id, port);
                        }
                    },
                    Err(_) => {
                        println!("{}", body);
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

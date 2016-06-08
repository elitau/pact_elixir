use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use std::io::prelude::*;
use rustc_serialize::json::Json;

pub fn list_mock_servers(host: &str, port: u16, matches: &ArgMatches) -> Result<(), i32> {
    let client = Client::new();
    let url = Url::parse(format!("http://{}:{}/", host, port).as_str()).unwrap();
    let res = client.get(url.clone()).send();

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
                        let provider_len = mock_servers.iter().fold(0, |acc, ref ms| {
                            let provider = ms.find("provider").unwrap().as_string().unwrap();
                            if provider.len() > acc {
                                provider.len()
                            } else {
                                acc
                            }
                        });

                        println!("{0:32}  {1:5}  {2:3$}  {4}", "Mock Server Id", "Port",
                            "Provider", provider_len, "Status");
                        for ms in mock_servers {
                            let id = ms.find("id").unwrap().as_string().unwrap();
                            let port = ms.find("port").unwrap();
                            let provider = ms.find("provider").unwrap().as_string().unwrap();
                            let status = ms.find("status").unwrap().as_string().unwrap();
                            println!("{0}  {1}  {2:3$}  {4}", id, port, provider, provider_len, status);
                        };
                        Ok(())
                    },
                    Err(err) => {
                        error!("Failed to parse JSON: {}\n{}", err, body);
                        ::display_error(format!("Failed to parse JSON: {}\n{}", err, body), matches);
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

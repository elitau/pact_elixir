use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::prelude::*;
use std::path::Path;
use serde_json;
use pact_matching::models::Pact;

pub fn create_mock_server(host: &str, port: u16, matches: &ArgMatches) -> Result<(), i32> {
    let file = matches.value_of("file").unwrap();
    info!("Creating mock server from file {}", file);

    match Pact::read_pact(&Path::new(file)) {
        Ok(ref pact) => {
            let client = Client::new();
            let url = Url::parse(format!("http://{}:{}/", host, port).as_str()).unwrap();
            let res = client.post(url.clone())
                .body(&pact.to_json().to_string())
                .header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)])))
                .send();

            match res {
                Ok(mut result) => {
                    let mut body = String::new();
                    result.read_to_string(&mut body).unwrap();
                    if result.status.is_success() {
                        let json_result: Result<serde_json::Value, _> = serde_json::from_str(body.as_str());
                        match json_result {
                            Ok(json) => {
                                let mock_server = json.get("mockServer").unwrap();
                                let id = mock_server.get("id").unwrap();
                                let port = mock_server.get("port").unwrap();
                                println!("Mock server {} started on port {}", id, port);
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
        },
        Err(err) => {
            ::display_error(format!("Failed to load pact file '{}': {}", file, err), matches);
        }
    }
}

use clap::ArgMatches;
use hyper::Client;
use hyper::Url;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json::Json;

fn read_pact_file(file: &str) -> io::Result<String> {
    let mut f = try!(File::open(file));
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));
    Ok(buffer)
}

pub fn create_mock_server(host: &str, port: u16, matches: &ArgMatches) {
    let file = matches.value_of("file").unwrap();
    info!("Creating mock server from file {}", file);

    match read_pact_file(file) {
        Ok(ref pact) => {
            let client = Client::new();
            let url = Url::parse(format!("http://{}:{}/", host, port).as_str()).unwrap();
            let res = client.post(url.clone())
                .body(pact)
                .header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)])))
                .send();

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
                                println!("Mock server {} started on port {}", id, port);
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
        },
        Err(err) => {
            ::display_error(format!("Failed to load pact file '{}': {}", file, err), matches);
        }
    }
}

use pact_matching::models::Pact;
use rustc_serialize::json::Json;
use itertools::Itertools;
use std::collections::BTreeMap;
use hyper::client::*;
use std::error::Error;
use super::provider_client::join_paths;
use hyper::header::{Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

fn is_true(object: &BTreeMap<String, Json>, field: &String) -> bool {
    match object.get(field) {
        Some(json) => match json {
            &Json::Boolean(b) => b,
            _e => false
        },
        None => false
    }
}

fn as_string(json: &Json) -> String {
    match json {
        &Json::String(ref s) => s.clone(),
        _ => format!("{}", json)
    }
}

pub struct HALClient {
    url: String,
    provider: String,
    path_info: Option<Json>
}

impl HALClient {

    fn default() -> HALClient {
        HALClient{ url: s!(""), provider: s!(""), path_info: None }
    }

    fn navigate(&mut self, link: &str) -> Result<(), String> {
        if self.path_info.is_none() {
            self.path_info = Some(try!(self.fetch("/")));
        }
        self.path_info = Some(try!(self.fetch_link(link)));
        Ok(())
    }

    fn fetch_link(&self, link: &str) -> Result<Json, String> {
        let link_data = try!(match self.path_info {
            None => Err(format!("Expected a HAL+JSON response from the pact broker, but got a non-JSON response. URL: '{}', LINK: '{}'",
                self.url, link)),
            Some(ref json) => match json.find("_links") {
                Some(json) => match json.find(link) {
                    Some(link_data) => link_data.as_object()
                        .ok_or(format!("Link is malformed, expcted an object but got {}. URL: '{}', LINK: '{}'",
                            link_data, self.url, link)),
                    None => Err(format!("Link '{}' was not found in the response, only the following links where found: {:?}. URL: '{}', LINK: '{}'",
                        link, json.as_object().unwrap().keys().join(", "), self.url, link))
                },
                None => Err(format!("Expected a HAL+JSON response from the pact broker, but got a response with no '_links'. URL: '{}', LINK: '{}'",
                    self.url, link))
            }
        });

        let link_url = try!(if is_true(link_data, &s!("templated")) {
            self.parse_link_url(link_data)
        } else {
            link_data.get("href").map(|href| as_string(href)).ok_or(format!("Link is malformed, there is no href. URL: '{}', LINK: '{}'",
                self.url, link))
        });
        self.fetch(&link_url)
    }

    fn fetch(&self, path: &str) -> Result<Json, String> {
        debug!("Fetching path '{}' from pact broker", path);
        let client = Client::new();
        let res = client.get(&join_paths(self.url.clone(), s!(path)))
            .header(Accept(vec![
                qitem(Mime(TopLevel::Application, SubLevel::Ext(s!("hal+json")),
                   vec![(Attr::Charset, Value::Utf8)])),
               qitem(Mime(TopLevel::Application, SubLevel::Json,
                  vec![(Attr::Charset, Value::Utf8)]))
            ]))
            .send();
        match res {
            Ok(response) => {
                debug!("Got response {:?}", response);
                if response.status.is_success() {
                    Err(s!(""))
                } else {
                    Err(format!("Request to pact broker path '{}' failed: {}", path,
                        response.status))
                }
            },
            Err(err) => Err(format!("Failed to access pact broker path '{}' - {:?}. URL: '{}'",
                path, err.description(), self.url))
        }
    }

    fn parse_link_url(&self, link_data: &BTreeMap<String, Json>) -> Result<String, String> {
        Err(s!(""))
    }
}

pub fn fetch_pacts_from_broker(broker_url: &String, provider_name: &String) -> Result<Vec<Result<Pact, String>>, String> {
    let mut client = HALClient{ url: broker_url.clone(), provider: provider_name.clone(), .. HALClient::default() };
    client.navigate("pb:latest-provider-pacts");
    Err(s!("Boom"))
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::*;
    use pact_consumer::*;
    use env_logger::*;

    #[test]
    fn fetch_returns_an_error_if_there_is_no_pact_broker() {
        let client = HALClient{ url: s!("http://idont.exist:6666"), provider: s!("sad_provider"), .. HALClient::default() };
        expect!(client.fetch(&s!("/"))).to(be_err());
    }

    #[test]
    fn fetch_returns_an_error_if_it_does_not_get_a_success_response() {
        init().unwrap_or(());

        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBroker"))
            .given(s!("the pact broker has a valid pact"))
            .upon_receiving(s!("a request to a non-existant path"))
                .path(s!("/hello"))
            .will_respond_with()
                .status(404)
            .build();

        let result = pact_runner.run(&|broker_url| {
            debug!("Broker URL is {}, Running test ...", broker_url);
            let client = HALClient{ url: broker_url, provider: s!("sad_provider"), .. HALClient::default() };
            let result = client.fetch(&s!("/hello"));
            expect!(result).to(be_err().value(s!("Request to pact broker path \'/hello\' failed: 404 Not Found")));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_returns_an_error_if_it_does_not_get_a_hal_response() {
        let client = HALClient{ url: s!("http://idont.exist:6666"), provider: s!("sad_provider"), .. HALClient::default() };
        expect!(client.fetch(&s!("/"))).to(be_err());
    }
}

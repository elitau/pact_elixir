use pact_matching::models::{Pact, OptionalBody};
use rustc_serialize::json::Json;
use itertools::Itertools;
use std::collections::BTreeMap;
use hyper::client::*;
use std::error::Error;
use super::provider_client::join_paths;
use hyper::header::{Accept, qitem, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use provider_client::extract_body;

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

fn content_type(response: &Response) -> String {
    match response.headers.get::<ContentType>() {
        Some(header) => format!("{}", header),
        None => s!("text/plain")
    }
}

fn json_content_type(response: &Response) -> bool {
    match response.headers.get::<ContentType>() {
        Some(header) => {
            let &ContentType(ref mime) = header;
            match mime.clone() {
                Mime(TopLevel::Application, SubLevel::Json, _) => true,
                Mime(TopLevel::Application, SubLevel::Ext(ext), _) => ext == "hal+json",
                _ => false
            }
        },
        None => false
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
            Ok(mut response) => {
                if response.status.is_success() {
                    if json_content_type(&response) {
                        match extract_body(&mut response) {
                            OptionalBody::Present(body) => Json::from_str(&body)
                                .map_err(|err| format!("Did not get a valid HAL response body from pact broker path '{}' - {}: {}. URL: '{}'",
                                    path, err.description(), err, self.url)),
                            _ => Err(format!("Did not get a valid HAL response body from pact broker path '{}'. URL: '{}'",
                                path, self.url))
                        }
                    } else {
                        Err(format!("Did not get a HAL response from pact broker path '{}', content type is '{}'. URL: '{}'",
                            path, content_type(&response), self.url))
                    }
                } else {
                    Err(format!("Request to pact broker path '{}' failed: {}. URL: '{}'", path,
                        response.status, self.url))
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
    use super::{content_type, json_content_type};
    use pact_consumer::*;
    use env_logger::*;
    use pact_matching::models::OptionalBody;
    use hyper::Url;
    use hyper::client::response::Response;
    use std::io::{self, Write, Read};
    use hyper::http::{
        RawStatus,
        HttpMessage,
        RequestHead,
        ResponseHead,
    };
    use hyper::error::Error;
    use hyper::version::HttpVersion;
    use std::time::Duration;
    use hyper::header::{Headers, ContentType};
    use std::borrow::Cow;
    use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

    #[test]
    fn fetch_returns_an_error_if_there_is_no_pact_broker() {
        let client = HALClient{ url: s!("http://idont.exist:6666"), provider: s!("sad_provider"), .. HALClient::default() };
        expect!(client.fetch(&s!("/"))).to(be_err());
    }

    #[test]
    fn fetch_returns_an_error_if_it_does_not_get_a_success_response() {
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBroker"))
            .given(s!("the pact broker has a valid pact"))
            .upon_receiving(s!("a request to a non-existant path"))
                .path(s!("/hello"))
            .will_respond_with()
                .status(404)
            .build();

        let result = pact_runner.run(&|broker_url| {
            let client = HALClient{ url: broker_url.clone(), provider: s!("sad_provider"), .. HALClient::default() };
            let result = client.fetch(&s!("/hello"));
            expect!(result).to(be_err().value(format!("Request to pact broker path \'/hello\' failed: 404 Not Found. URL: '{}'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_returns_an_error_if_it_does_not_get_a_hal_response() {
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
            .upon_receiving(s!("a request to a non-json resource"))
                .path(s!("/nonjson"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("text/html") })
                .body(OptionalBody::Present(s!("<html></html>")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let client = HALClient{ url: broker_url.clone(), provider: s!("sad_provider"), .. HALClient::default() };
            let result = client.fetch(&s!("/nonjson"));
            expect!(result).to(be_err().value(format!("Did not get a HAL response from pact broker path \'/nonjson\', content type is 'text/html'. URL: '{}'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[derive(Debug, Clone)]
    struct MockHttpMessage {
        pub body: Option<String>,
        pub headers: Headers,
        pub status: RawStatus
    }

    impl HttpMessage for MockHttpMessage {

        fn set_outgoing(&mut self, _head: RequestHead) -> Result<RequestHead, Error> {
            Err(Error::Io(io::Error::new(io::ErrorKind::Other, "Not supported with MockHttpMessage")))
        }

        fn get_incoming(&mut self) -> Result<ResponseHead, Error> {
            Ok(ResponseHead {
                headers: self.headers.clone(),
                raw_status: self.status.clone(),
                version: HttpVersion::Http11,
            })
        }

        fn has_body(&self) -> bool {
            self.body.is_some()
        }

        fn set_read_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
            Ok(())
        }

        fn set_write_timeout(&self, _dur: Option<Duration>) -> io::Result<()> {
            Ok(())
        }

        fn close_connection(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    impl Write for MockHttpMessage {

        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Not supported with MockHttpMessage"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "Not supported with MockHttpMessage"))
        }

    }

    impl Read for MockHttpMessage {

        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Not supported with MockHttpMessage"))
        }

    }

    #[test]
    fn content_type_test() {
        let mut message = MockHttpMessage {
            body: None,
            status: RawStatus(200, Cow::Owned(s!("OK"))),
            headers: Headers::new()
        };
        let url = Url::parse("http://localhost").unwrap();

        let response = Response::with_message(url.clone(), Box::new(message.clone())).unwrap();
        expect!(content_type(&response)).to(be_equal_to(s!("text/plain")));

        message.headers.set::<ContentType>(
            ContentType(Mime(TopLevel::Application, SubLevel::Ext(s!("hal+json")),
                vec![(Attr::Charset, Value::Utf8)])));
        let response = Response::with_message(url.clone(), Box::new(message.clone())).unwrap();
        expect!(content_type(&response)).to(be_equal_to(s!("application/hal+json; charset=utf-8")));
    }

    #[test]
    fn json_content_type_test() {
        let mut message = MockHttpMessage {
            body: None,
            status: RawStatus(200, Cow::Owned(s!("OK"))),
            headers: Headers::new()
        };
        let url = Url::parse("http://localhost").unwrap();

        let response = Response::with_message(url.clone(), Box::new(message.clone())).unwrap();
        expect!(json_content_type(&response)).to(be_false());

        message.headers.set::<ContentType>(
            ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
        let response = Response::with_message(url.clone(), Box::new(message.clone())).unwrap();
        expect!(json_content_type(&response)).to(be_true());

        message.headers.set::<ContentType>(
            ContentType(Mime(TopLevel::Application, SubLevel::Ext(s!("hal+json")),
                vec![(Attr::Charset, Value::Utf8)])));
        let response = Response::with_message(url.clone(), Box::new(message.clone())).unwrap();
        expect!(json_content_type(&response)).to(be_true());
    }

    #[test]
    fn fetch_returns_an_error_if_it_does_not_get_a_valid_hal_response() {
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
            .upon_receiving(s!("a request to a non-hal resource"))
                .path(s!("/nonhal"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
            .upon_receiving(s!("a request to a non-hal resource 2"))
                .path(s!("/nonhal2"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("<html>This is not JSON</html>")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let client = HALClient{ url: broker_url.clone(), provider: s!("sad_provider"), .. HALClient::default() };
            let result = client.fetch(&s!("/nonhal"));
            expect!(result).to(be_err().value(format!("Did not get a valid HAL response body from pact broker path \'/nonhal\'. URL: '{}'",
                broker_url)));
            let result = client.fetch(&s!("/nonhal2"));
            expect!(result).to(be_err().value(format!("Did not get a valid HAL response body from pact broker path \'/nonhal2\' - failed to parse json: SyntaxError(\"invalid syntax\", 1, 1). URL: '{}'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }
}

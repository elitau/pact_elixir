use pact_matching::models::{Pact, OptionalBody};
use rustc_serialize::json::Json;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use hyper::client::*;
use std::error::Error;
use super::provider_client::join_paths;
use hyper::header::{Accept, qitem, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use provider_client::extract_body;
use regex::{Regex, Captures};
use hyper::Url;
use hyper::status::StatusCode;

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

fn find_entry<T>(map: &BTreeMap<String, T>, key: &String) -> Option<(String, T)> where T: Clone {
    match map.keys().find(|k| k.to_lowercase() == key.to_lowercase() ) {
        Some(k) => map.get(k).map(|v| (key.clone(), v.clone()) ),
        None => None
    }
}

#[derive(Debug, Clone)]
pub enum PactBrokerError {
    LinkError(String),
    ContentError(String),
    IoError(String),
    NotFound(String),
    UrlError(String)
}

impl PartialEq<String> for PactBrokerError {
    fn eq(&self, other: &String) -> bool {
        let message = match self {
            &PactBrokerError::LinkError(ref s) => s.clone(),
            &PactBrokerError::ContentError(ref s) => s.clone(),
            &PactBrokerError::IoError(ref s) => s.clone(),
            &PactBrokerError::NotFound(ref s) => s.clone(),
            &PactBrokerError::UrlError(ref s) => s.clone()
        };
        message == *other
    }
}

impl <'a> PartialEq<&'a str> for PactBrokerError {
    fn eq(&self, other: &&str) -> bool {
        let message = match self {
            &PactBrokerError::LinkError(ref s) => s.clone(),
            &PactBrokerError::ContentError(ref s) => s.clone(),
            &PactBrokerError::IoError(ref s) => s.clone(),
            &PactBrokerError::NotFound(ref s) => s.clone(),
            &PactBrokerError::UrlError(ref s) => s.clone()
        };
        message.as_str() == *other
    }
}

pub struct HALClient {
    url: String,
    path_info: Option<Json>
}

impl HALClient {

    fn default() -> HALClient {
        HALClient{ url: s!(""), path_info: None }
    }

    fn navigate(&mut self, link: &str, template_values: &HashMap<String, String>) -> Result<(), PactBrokerError> {
        if self.path_info.is_none() {
            self.path_info = Some(try!(self.fetch("/")));
        }
        self.path_info = Some(try!(self.fetch_link(link, template_values)));
        Ok(())
    }

    fn fetch_link(&self, link: &str, template_values: &HashMap<String, String>) -> Result<Json, PactBrokerError> {
        let link_data = try!(match self.path_info {
            None => Err(PactBrokerError::LinkError(format!("No previous resource has been fetched from the pact broker. URL: '{}', LINK: '{}'",
                self.url, link))),
            Some(ref json) => match json.find("_links") {
                Some(json) => match json.find(link) {
                    Some(link_data) => link_data.as_object()
                        .ok_or(PactBrokerError::LinkError(format!("Link is malformed, expcted an object but got {}. URL: '{}', LINK: '{}'",
                            link_data, self.url, link))),
                    None => Err(PactBrokerError::LinkError(format!("Link '{}' was not found in the response, only the following links where found: {:?}. URL: '{}', LINK: '{}'",
                        link, json.as_object().unwrap_or(&btreemap!{}).keys().join(", "), self.url, link)))
                },
                None => Err(PactBrokerError::LinkError(format!("Expected a HAL+JSON response from the pact broker, but got a response with no '_links'. URL: '{}', LINK: '{}'",
                    self.url, link)))
            }
        });

        let link_url = try!(if is_true(link_data, &s!("templated")) {
            debug!("Link URL is templated");
            self.parse_link_url(link_data, template_values, link)
        } else {
            find_entry(link_data, &s!("href")).map(|(_, href)| as_string(&href))
                .ok_or(PactBrokerError::LinkError(format!("Link is malformed, there is no href. URL: '{}', LINK: '{}'",
                    self.url, link)))
        });
        let base = try!(Url::parse(&self.url).map_err(|err| PactBrokerError::UrlError(format!("{}", err.description()))));
        let url = try!(base.join(&link_url).map_err(|err| PactBrokerError::UrlError(format!("{}", err.description()))));
        self.fetch(&url.path())
    }

    fn fetch(&self, path: &str) -> Result<Json, PactBrokerError> {
        debug!("Fetching path '{}' from pact broker", path);
        let client = Client::new();
        let res = client.get(&join_paths(self.url.clone(), s!(path)))
            .header(Accept(vec![
                qitem(Mime(TopLevel::Application, SubLevel::Ext(s!("hal+json")),
                   vec![(Attr::Charset, Value::Utf8)]))
            ]))
            .send();
        match res {
            Ok(mut response) => {
                if response.status.is_success() {
                    if json_content_type(&response) {
                        match extract_body(&mut response) {
                            OptionalBody::Present(body) => Json::from_str(&body)
                                .map_err(|err| PactBrokerError::ContentError(format!("Did not get a valid HAL response body from pact broker path '{}' - {}: {}. URL: '{}'",
                                    path, err.description(), err, self.url))),
                            _ => Err(PactBrokerError::ContentError(format!("Did not get a valid HAL response body from pact broker path '{}'. URL: '{}'",
                                path, self.url)))
                        }
                    } else {
                        Err(PactBrokerError::ContentError(format!("Did not get a HAL response from pact broker path '{}', content type is '{}'. URL: '{}'",
                            path, content_type(&response), self.url)))
                    }
                } else {
                    if response.status == StatusCode::NotFound {
                        Err(PactBrokerError::NotFound(format!("Request to pact broker path '{}' failed: {}. URL: '{}'", path,
                            response.status, self.url)))
                    } else {
                        Err(PactBrokerError::IoError(format!("Request to pact broker path '{}' failed: {}. URL: '{}'", path,
                            response.status, self.url)))
                    }
                }
            },
            Err(err) => Err(PactBrokerError::IoError(format!("Failed to access pact broker path '{}' - {:?}. URL: '{}'",
                path, err.description(), self.url)))
        }
    }

    fn parse_link_url(&self, link_data: &BTreeMap<String, Json>, values: &HashMap<String, String>, link: &str) -> Result<String, PactBrokerError> {
        match find_entry(link_data, &s!("href")) {
            Some((_, value)) => {
                let href_template = as_string(&value);
                debug!("templated URL = {}", href_template);
                let re = Regex::new(r"\{(\w+)\}").unwrap();
                let final_url = re.replace_all(&href_template, |caps: &Captures| {
                    let lookup = caps.at(1).unwrap();
                    debug!("Looking up value for key '{}'", lookup);
                    match values.get(lookup) {
                        Some(val) => val.clone(),
                        None => {
                            warn!("No value was found for key '{}', mapped values are {:?}",
                                lookup, values);
                            format!("{{{}}}", lookup)
                        }
                    }
                });
                debug!("final URL = {}", final_url);
                Ok(final_url)
            },
            None => Err(PactBrokerError::LinkError(format!("Expected a HAL+JSON response from the pact broker, but got a link with no HREF. URL: '{}', LINK: '{}'",
                self.url, link)))
        }
    }
}

pub fn fetch_pacts_from_broker(broker_url: &String, provider_name: &String) -> Result<Vec<Result<Pact, String>>, PactBrokerError> {
    let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
    match client.navigate("pb:latest-provider-pacts", &hashmap!{ s!("provider") => provider_name.clone() }) {
        Ok(stuff) => Err(PactBrokerError::LinkError(s!("Boom"))),
        Err(err) => match err {
            PactBrokerError::NotFound(_) => Err(
                PactBrokerError::NotFound(
                    format!("No pacts for provider '{}' where found in the pact broker. URL: '{}'",
                        provider_name, broker_url))),
            _ => Err(err)
        }
    }
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
    use rustc_serialize::json::Json;

    #[test]
    fn fetch_returns_an_error_if_there_is_no_pact_broker() {
        let client = HALClient{ url: s!("http://idont.exist:6666"), .. HALClient::default() };
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
            let client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
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
            let client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
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
            let client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
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

    #[test]
    fn parse_link_url_returns_error_if_there_is_no_href() {
        let client = HALClient::default();
        expect!(client.parse_link_url(&btreemap!{}, &hashmap!{}, "link")).to(be_err().value(
            "Expected a HAL+JSON response from the pact broker, but got a link with no HREF. URL: '', LINK: 'link'"));
    }

    #[test]
    fn parse_link_url_replaces_all_tokens_in_href() {
        let client = HALClient::default();
        let values = hashmap!{ s!("valA") => s!("A"), s!("valB") => s!("B") };

        expect!(client.parse_link_url(&btreemap!{ s!("href") => Json::String(s!("http://localhost")) },
            &values, "link"))
        .to(be_ok().value("http://localhost"));

        expect!(client.parse_link_url(&btreemap!{ s!("hRef") => Json::String(s!("http://{valA}/{valB}")) },
            &values, "link"))
        .to(be_ok().value("http://A/B"));

        expect!(client.parse_link_url(&btreemap!{ s!("HREF") => Json::String(s!("http://{valA}/{valC}")) },
            &values, "link"))
        .to(be_ok().value("http://A/{valC}"));
    }

    #[test]
    fn fetch_link_returns_an_error_if_a_previous_resource_has_not_been_fetched() {
        let client = HALClient{ url: s!("http://localhost"), .. HALClient::default() };
        let result = client.fetch_link(&s!("anything_will_do"), &hashmap!{});
        expect!(result).to(be_err().value(s!("No previous resource has been fetched from the pact broker. URL: 'http://localhost', LINK: 'anything_will_do'")));
    }

    #[test]
    fn fetch_link_returns_an_error_if_the_previous_resource_was_not_hal() {
        init().unwrap_or(());
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
                .upon_receiving(s!("a request to a non-hal json resource"))
                .path(s!("/"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("{}")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
            let result = client.fetch(&s!("/"));
            expect!(result.clone()).to(be_ok());
            client.path_info = result.ok();
            let result = client.fetch_link(&s!("hal2"), &hashmap!{});
            expect!(result).to(be_err().value(format!("Expected a HAL+JSON response from the pact broker, but got a response with no '_links'. URL: '{}', LINK: 'hal2'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_link_returns_an_error_if_the_previous_resource_links_are_not_correctly_formed() {
        init().unwrap_or(());
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
                .upon_receiving(s!("a request to a hal resource with invalid links"))
                .path(s!("/"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("{\"_links\":[{\"next\":{\"href\":\"abc\"}},{\"prev\":{\"href\":\"def\"}}]}")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
            let result = client.fetch(&s!("/"));
            expect!(result.clone()).to(be_ok());
            client.path_info = result.ok();
            let result = client.fetch_link(&s!("any"), &hashmap!{});
            expect!(result).to(be_err().value(format!("Link 'any' was not found in the response, only the following links where found: \"\". URL: '{}', LINK: 'any'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_link_returns_an_error_if_the_previous_resource_does_not_have_the_link() {
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
                .upon_receiving(s!("a request to a hal resource"))
                .path(s!("/"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("{\"_links\":{\"next\":{\"href\":\"/abc\"},\"prev\":{\"href\":\"/def\"}}}")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
            let result = client.fetch(&s!("/"));
            expect!(result.clone()).to(be_ok());
            client.path_info = result.ok();
            let result = client.fetch_link(&s!("any"), &hashmap!{});
            expect!(result).to(be_err().value(format!("Link 'any' was not found in the response, only the following links where found: \"next, prev\". URL: '{}', LINK: 'any'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_link_returns_the_resource_for_the_link() {
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
            .upon_receiving(s!("a request to a hal resource"))
                .path(s!("/"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("{\"_links\":{\"next\":{\"href\":\"/abc\"},\"prev\":{\"href\":\"/def\"}}}")))
            .upon_receiving(s!("a request to next"))
                .path(s!("/abc"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/json") })
                .body(OptionalBody::Present(s!("\"Yay! You found your way here\"")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
            let result = client.fetch(&s!("/"));
            expect!(result.clone()).to(be_ok());
            client.path_info = result.ok();
            let result = client.fetch_link(&s!("next"), &hashmap!{});
            expect!(result).to(be_ok().value(Json::String(s!("Yay! You found your way here"))));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_link_returns_handles_absolute_resource_links() {
        init().unwrap_or(());
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
            .upon_receiving(s!("a request to a hal resource with absolute paths"))
                .path(s!("/"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("{\"_links\":{\"next\":{\"href\":\"http://localhost/abc\"},\"prev\":{\"href\":\"http://localhost/def\"}}}")))
            .upon_receiving(s!("a request to next"))
                .path(s!("/abc"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/json") })
                .body(OptionalBody::Present(s!("\"Yay! You found your way here\"")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
            let result = client.fetch(&s!("/"));
            expect!(result.clone()).to(be_ok());
            client.path_info = result.ok();
            let result = client.fetch_link(&s!("next"), &hashmap!{});
            expect!(result).to(be_ok().value(Json::String(s!("Yay! You found your way here"))));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_link_returns_the_resource_for_the_templated_link() {
        init().unwrap_or(());
        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBrokerStub"))
            .upon_receiving(s!("a request to a templated hal resource"))
                .path(s!("/"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!("{\"_links\":{\"document\":{\"href\":\"/doc/{id}\",\"templated\":true}}}")))
            .upon_receiving(s!("a request for a document"))
                .path(s!("/doc/abc"))
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/json") })
                .body(OptionalBody::Present(s!("\"Yay! You found your way here\"")))
            .build();

        let result = pact_runner.run(&|broker_url| {
            let mut client = HALClient{ url: broker_url.clone(), .. HALClient::default() };
            let result = client.fetch(&s!("/"));
            expect!(result.clone()).to(be_ok());
            client.path_info = result.ok();
            let result = client.fetch_link(&s!("document"), &hashmap!{ s!("id") => s!("abc") });
            expect!(result).to(be_ok().value(Json::String(s!("Yay! You found your way here"))));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }

    #[test]
    fn fetch_pacts_from_broker_returns_empty_list_if_there_are_no_pacts() {
        init().unwrap_or(());

        let pact_runner = ConsumerPactBuilder::consumer(s!("RustPactVerifier"))
            .has_pact_with(s!("PactBroker"))
            .upon_receiving(s!("a request to the pact broker root"))
                .path(s!("/"))
                .headers(hashmap!{ s!("Accept") => s!("application/hal+json; charset=utf-8") })
            .will_respond_with()
                .status(200)
                .headers(hashmap!{ s!("Content-Type") => s!("application/hal+json") })
                .body(OptionalBody::Present(s!(r#"
                    {
                        "_links":{
                            "pb:latest-provider-pacts":{"href":"http://localhost/pacts/provider/{provider}/latest","templated":true}
                        }
                    }
                "#)))
            .given(s!("There are no pacts in the pact broker"))
            .upon_receiving(s!("a request for a providers pacts"))
                .path(s!("/pacts/provider/sad_provider/latest"))
                .headers(hashmap!{ s!("Accept") => s!("application/hal+json; charset=utf-8") })
            .will_respond_with()
                .status(404)
            .build();

        let result = pact_runner.run(&|broker_url| {
            let result = fetch_pacts_from_broker(&broker_url, &s!("sad_provider"));
            expect!(result).to(be_err().value(format!("No pacts for provider 'sad_provider' where found in the pact broker. URL: '{}'",
                broker_url)));
            Ok(())
        });
        expect!(result).to(be_equal_to(VerificationResult::PactVerified));
    }
}

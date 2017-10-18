use super::*;
use pact_matching::models::*;
use pact_matching::models::matchingrules::MatchingRules;
use std::str::FromStr;
use std::collections::hash_map::HashMap;
use std::io::Read;
use hyper::client::Client;
use hyper::client::response::Response as HyperResponse;
use hyper::error::Error as HyperError;
use hyper::method::Method;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

pub fn join_paths(base: &String, path: String) -> String {
    let mut full_path = s!(base.trim_right_matches("/"));
    full_path.push('/');
    full_path.push_str(path.trim_left_matches("/"));
    full_path
}

fn setup_headers(headers: &Option<HashMap<String, String>>) -> Headers {
    let mut hyper_headers = Headers::new();
    if headers.is_some() {
        let headers = headers.clone().unwrap();
        for (k, v) in headers.clone() {
            hyper_headers.set_raw(k.clone(), vec![v.bytes().collect()]);
        }

        if !headers.keys().any(|k| k.to_lowercase() == "content-type") {
            hyper_headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                vec![])));
        }
    }
    hyper_headers
}

fn make_request(base_url: &String, request: &Request, client: &Client) -> Result<HyperResponse, HyperError> {
    match Method::from_str(&request.method) {
        Ok(method) => {
            let mut url = join_paths(base_url, request.path.clone());
            if request.query.is_some() {
                url.push('?');
                url.push_str(&build_query_string(request.query.clone().unwrap()));
            }
            debug!("Making request to '{}'", url);
            let hyper_request = client.request(method, &url)
                .headers(setup_headers(&request.headers.clone()));
            match request.body {
                OptionalBody::Present(ref s) => hyper_request.body(s.as_str()),
                OptionalBody::Null => {
                    if request.content_type() == "application/json" {
                        hyper_request.body("null")
                    } else {
                        hyper_request
                    }
                },
                _ => hyper_request
            }.send()
        },
        Err(err) => Err(err)
    }
}

fn extract_headers(headers: &Headers) -> Option<HashMap<String, String>> {
    if headers.len() > 0 {
        Some(headers.iter().map(|h| (s!(h.name()), h.value_string()) ).collect())
    } else {
        None
    }
}

pub fn extract_body(response: &mut HyperResponse) -> OptionalBody {
    let mut buffer = String::new();
    match response.read_to_string(&mut buffer) {
        Ok(size) => if size > 0 {
                OptionalBody::Present(buffer)
            } else {
                OptionalBody::Empty
            },
        Err(err) => {
            warn!("Failed to read request body: {}", err);
            OptionalBody::Missing
        }
    }
}

fn hyper_response_to_pact_response(response: &mut HyperResponse) -> Response {
    Response {
        status: response.status.to_u16(),
        headers: extract_headers(&response.headers),
        body: extract_body(response),
        matching_rules: MatchingRules::default()
    }
}

pub fn make_provider_request(provider: &ProviderInfo, request: &Request) -> Result<Response, HyperError> {
    debug!("Sending {:?} to provider", request);
    let client = Client::new();
    match make_request(&format!("{}://{}:{}{}", provider.protocol, provider.host, provider.port,
        provider.path), request, &client) {
        Ok(ref mut response) => {
            debug!("Received response: {:?}", response);
            Ok(hyper_response_to_pact_response(response))
        },
        Err(err) => {
            debug!("Request failed: {}", err);
            Err(err)
        }
    }
}

pub fn make_state_change_request(provider: &ProviderInfo, request: &Request) -> Result<(), String> {
    debug!("Sending {:?} to state change handler", request);
    let client = Client::new();
    match make_request(&provider.state_change_url.clone().unwrap(), request, &client) {
        Ok(ref mut response) => {
            debug!("Received response: {:?}", response);
            if response.status.is_success() {
                Ok(())
            } else {
                debug!("Request failed: {}", response.status);
                Err(format!("State change request failed: {}", response.status))
            }
        },
        Err(err) => {
            debug!("Request failed: {}", err);
            Err(format!("State change request failed: {}", err))
        }
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::join_paths;

    #[test]
    fn join_paths_test() {
        expect!(join_paths(&s!(""), s!(""))).to(be_equal_to(s!("/")));
        expect!(join_paths(&s!("/"), s!(""))).to(be_equal_to(s!("/")));
        expect!(join_paths(&s!(""), s!("/"))).to(be_equal_to(s!("/")));
        expect!(join_paths(&s!("/"), s!("/"))).to(be_equal_to(s!("/")));
        expect!(join_paths(&s!("/a/b"), s!("/c/d"))).to(be_equal_to(s!("/a/b/c/d")));
    }

}

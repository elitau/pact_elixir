use super::*;
use pact_matching::models::*;
use std::io;
use std::str::FromStr;
use std::collections::hash_map::HashMap;
use hyper::client::{Client, RequestBuilder};
use hyper::client::response::Response as HyperResponse;
use hyper::error::Error;
use hyper::method::Method;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

fn join_paths(base: String, path: String) -> String {
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

fn make_request(provider: &ProviderInfo, request: &Request, client: &Client) -> Result<HyperResponse, Error> {
    match Method::from_str(&request.method) {
        Ok(method) => {
            let base_url = format!("{}://{}:{}{}", provider.protocol, provider.host, provider.port,
                provider.path);
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
                    if request.mimetype() == "application/json" {
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

pub fn make_provider_request(provider: &ProviderInfo, request: &Request) -> Result<Response, Error> {
    debug!("Sending {:?} to provider", request);
    let client = Client::new();
    match make_request(provider, request, &client) {
        Ok(response) => {
            debug!("Received response: {:?}", response);
            Ok(Response::default_response())
        },
        Err(err) => {
            debug!("Request failed: {}", err);
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::join_paths;

    #[test]
    fn join_paths_test() {
        expect!(join_paths(s!(""), s!(""))).to(be_equal_to(s!("/")));
        expect!(join_paths(s!("/"), s!(""))).to(be_equal_to(s!("/")));
        expect!(join_paths(s!(""), s!("/"))).to(be_equal_to(s!("/")));
        expect!(join_paths(s!("/"), s!("/"))).to(be_equal_to(s!("/")));
        expect!(join_paths(s!("/a/b"), s!("/c/d"))).to(be_equal_to(s!("/a/b/c/d")));
    }

}

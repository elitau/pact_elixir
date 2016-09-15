use pact_matching::models::{Pact, Interaction};
use rustc_serialize::json::{Json, ToJson};

struct HALClient {
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
        match self.path_info {
            None => Err(format!("Expected a HAL+JSON response from the pact broker, but got a response with no '_links'. URL: '{}', LINK: '{}'",
                self.url, link)),
            Some(ref json) => Err(s!(""))
        }
    }

    fn fetch(&self, path: &str) -> Result<Json, String> {
        Err(s!(""))
    }
}

pub fn fetch_pacts_from_broker(broker_url: &String, provider_name: &String) -> Result<Vec<Result<Pact, String>>, String> {
    let mut client = HALClient{ url: broker_url.clone(), provider: provider_name.clone(), .. HALClient::default() };
    client.navigate("pb:latest-provider-pacts");
    Err(s!("Boom"))
}

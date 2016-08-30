//! The `pact_verifier` crate provides the core logic to performing verification of providers.
//! It implements the V1 Pact specification (https://github.com/pact-foundation/pact-specification/tree/version-1).

#![warn(missing_docs)]

#[macro_use] extern crate pact_matching;
extern crate ansi_term;
#[macro_use] extern crate log;
extern crate hyper;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

mod provider_client;

use std::path::Path;
use pact_matching::models::{Pact, Interaction};
use ansi_term::Style;
use ansi_term::Colour::*;
use std::collections::HashMap;
use provider_client::make_provider_request;

/// Source for loading pacts
pub enum PactSource {
    /// Load the pact from a pact file
    File(String)
}

/// Information about the Provider to verify
pub struct ProviderInfo {
    /// Provider protocol, defaults to HTTP
    pub protocol: String,
    /// Hostname of the provider
    pub host: String,
    /// Port the provider is running on, defaults to 8080
    pub port: u16,
    /// Base path for the provider, defaults to /
    pub path: String
}

impl ProviderInfo {
    /// Create a default provider info
    pub fn default() -> ProviderInfo {
        ProviderInfo {
            protocol: s!("http"),
            host: s!("localhost"),
            port: 8080,
            path: s!("/")
        }
    }
}

fn verify_response_from_provider(provider: &ProviderInfo, interaction: &Interaction) -> Result<(), HashMap<String, Vec<String>>> {
    let expected_response = interaction.response.clone();
    let actual_response = make_provider_request(provider, &interaction.request);
    Ok(())
}

fn verify_interaction(provider: &ProviderInfo, interaction: &Interaction) -> Result<(), HashMap<String, Vec<String>>> {
    /*
    def interactionMessage = "Verifying a pact between ${consumer.name} and ${provider.name}" +
          " - ${interaction.description}"

        def stateChangeOk = true
        if (interaction.providerState) {
          stateChangeOk = stateChange(interaction.providerState, provider, consumer)
          log.debug "State Change: \"${interaction.providerState}\" -> ${stateChangeOk}"
          if (stateChangeOk != true) {
            failures[interactionMessage] = stateChangeOk
            stateChangeOk = false
          } else {
            interactionMessage += " Given ${interaction.providerState}"
          }
        }

        if (stateChangeOk) {*/

    println!("  {}", interaction.description);
    let result = verify_response_from_provider(provider, interaction);

    // if (provider.stateChangeTeardown) {
    // stateChange(interaction.providerState, provider, consumer, false)
    // }

    result
}

/// Verify the provider with the given pact source
pub fn verify_provider(provider: &ProviderInfo, source: PactSource) -> Result<(), String> {
    let pact = match source {
        PactSource::File(file) => {
            Pact::read_pact(Path::new(&file))
        }
    };
    match pact {
        Ok(pact) => {
            println!("\nVerifying a pact between {} and {}", Style::new().bold().paint(pact.consumer.name),
                Style::new().bold().paint(pact.provider.name));

            if pact.interactions.is_empty() {
                println!("         {}", Yellow.paint("WARNING: Pact file has no interactions"));
                Ok(())
            } else {
                let results: Vec<Result<(), HashMap<String, Vec<String>>>> = pact.interactions.iter().map(|interaction| {
                    verify_interaction(provider.clone(), interaction)
                }).collect();
                Ok(())
            }
        },
        Err(err) => {
            error!("Failed to load pact - {}", Red.paint(format!("{}", err)));
            Err(format!("{}", err))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

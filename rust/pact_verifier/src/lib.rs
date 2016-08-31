//! The `pact_verifier` crate provides the core logic to performing verification of providers.
//! It implements the V1 Pact specification (https://github.com/pact-foundation/pact-specification/tree/version-1).

#![warn(missing_docs)]

#[macro_use] extern crate pact_matching;
extern crate ansi_term;
#[macro_use] extern crate log;
extern crate hyper;
#[macro_use] extern crate maplit;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

mod provider_client;

use std::path::Path;
use std::error::Error;
use pact_matching::*;
use pact_matching::models::{Pact, Interaction};
use ansi_term::*;
use ansi_term::Colour::*;
use std::collections::HashMap;
use provider_client::make_provider_request;

/// Source for loading pacts
#[derive(Debug, Clone)]
pub enum PactSource {
    /// Load the pact from a pact file
    File(String)
}

/// Information about the Provider to verify
#[derive(Debug, Clone)]
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

/// Result of performing a match
#[derive(Debug, Clone)]
pub enum MismatchResult {
    /// Response mismatches
    Mismatches(Vec<Mismatch>),
    /// Error occured
    Error(String)
}

fn verify_response_from_provider(provider: &ProviderInfo, interaction: &Interaction) -> Result<(), MismatchResult> {
    let expected_response = interaction.response.clone();
    match make_provider_request(provider, &interaction.request) {
        Ok(actual_response) => {
            let mismatches = match_response(expected_response, actual_response);
            if mismatches.is_empty() {
                Ok(())
            } else {
                Err(MismatchResult::Mismatches(mismatches))
            }
        },
        Err(err) => {
            Err(MismatchResult::Error(s!(err.description())))
        }
    }
}

fn verify_interaction(provider: &ProviderInfo, interaction: &Interaction) -> Result<(), MismatchResult> {
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

    let result = verify_response_from_provider(provider, interaction);

    // if (provider.stateChangeTeardown) {
    // stateChange(interaction.providerState, provider, consumer, false)
    // }

    result
}

fn display_result(status: u16, status_result: ANSIGenericString<str>,
    header_results: Option<Vec<(String, String, ANSIGenericString<str>)>>,
    body_result: ANSIGenericString<str>) {
    println!("    returns a response which");
    println!("      has status code {} ({})", Style::new().bold().paint(format!("{}", status)),
        status_result);
    match header_results {
        Some(header_results) => {
            println!("      includes headers");
            for (key, value, result) in header_results {
                println!("        \"{}\" with value \"{}\" ({})", Style::new().bold().paint(key),
                    Style::new().bold().paint(value), result);
            }
        },
        None => ()
    }
    println!("      has a matching body ({})", body_result);
}

/// Verify the provider with the given pact source
pub fn verify_provider(provider_info: &ProviderInfo, source: PactSource) -> Result<(), ()> {
    let pact = match source {
        PactSource::File(file) => {
            Pact::read_pact(Path::new(&file))
        }
    };
    match pact {
        Ok(ref pact) => {
            println!("\nVerifying a pact between {} and {}",
                Style::new().bold().paint(pact.consumer.name.clone()),
                Style::new().bold().paint(pact.provider.name.clone()));

            if pact.interactions.is_empty() {
                println!("         {}", Yellow.paint("WARNING: Pact file has no interactions"));
                Ok(())
            } else {
                let results: HashMap<Interaction, Result<(), MismatchResult>> = pact.interactions.iter().map(|interaction| {
                    (interaction.clone(), verify_interaction(provider_info, interaction))
                }).collect();

                let mut return_result = Ok(());
                for (interaction, result) in results.clone() {
                    if interaction.provider_state.is_some() {
                        println!("  Given {}", Style::new().bold().paint(interaction.provider_state.unwrap()));
                    }
                    println!("  {}", interaction.description);
                    match result {
                        Ok(()) => {
                            display_result(interaction.response.status, Green.paint("OK"),
                                interaction.response.headers.map(|h| h.iter().map(|(k, v)| {
                                    (k.clone(), v.clone(), Green.paint("OK"))
                                }).collect()), Green.paint("OK"))
                        },
                        Err(err) => match err {
                            MismatchResult::Error(err) => {
                                println!("      {}", Red.paint(format!("Request Failed - {}", err)));
                                return_result = Err(());
                            },
                            MismatchResult::Mismatches(mismatches) => {
                                let mut iter = mismatches.iter();
                                let status_result = if iter.any(|m| m.mismatch_type() == s!("StatusMismatch")) {
                                    return_result = Err(());
                                    Red.paint("FAILED")
                                } else {
                                    Green.paint("OK")
                                };
                                let header_results = match interaction.response.headers {
                                    Some(ref h) => Some(h.iter().map(|(k, v)| {
                                        (k.clone(), v.clone(), if iter.any(|m| {
                                            match m {
                                                &Mismatch::HeaderMismatch{ ref key, .. } => k == key,
                                                _ => false
                                            }
                                        }) {
                                            return_result = Err(());
                                            Red.paint("FAILED")
                                        } else {
                                            Green.paint("OK")
                                        })
                                    }).collect()),
                                    None => None
                                };
                                let body_result = if iter.any(|m| m.mismatch_type() == s!("BodyMismatch") ||
                                    m.mismatch_type() == s!("BodyTypeMismatch")) {
                                    return_result = Err(());
                                    Red.paint("FAILED")
                                } else {
                                    Green.paint("OK")
                                };

                                display_result(interaction.response.status, status_result, header_results,
                                    body_result);
                            }
                        }
                    }
                }
                println!("");

                let all_errors: Vec<(Interaction, MismatchResult)> = results.iter()
                    .filter(|&(_, v)| v.is_err())
                    .map(|(k, v)| (k.clone(), v.clone().unwrap_err())).collect();

                if !all_errors.is_empty() {
                    println!("Failures:\n");

                    for (i, &(ref interaction, ref mismatch)) in all_errors.iter().enumerate() {
                        let mut description = format!("Verifying a pact between {} and {}",
                            pact.consumer.name.clone(), pact.provider.name.clone());
                        if interaction.provider_state.is_some() {
                            description.push_str(&format!("  Given {}",
                                interaction.provider_state.clone().unwrap()));
                        }
                        println!("{}) {}", i, description);
                    }

                    println!("\nThere were {} pact failures for provider {}\n", all_errors.len(),
                        pact.provider.name.clone());
                }

                return_result
            }
        },
        Err(err) => {
            error!("Failed to load pact - {}", Red.paint(format!("{}", err)));
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

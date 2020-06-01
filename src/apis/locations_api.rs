/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct LocationsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl LocationsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> LocationsApiClient {
        LocationsApiClient {
            configuration,
        }
    }
}

pub trait LocationsApi {
    fn get_location(&self, id: &str) -> Result<crate::models::GetLocationResponse, Error>;
    fn list_locations(&self, name: Option<&str>) -> Result<crate::models::ListLocationsResponse, Error>;
}

impl LocationsApi for LocationsApiClient {
    fn get_location(&self, id: &str) -> Result<crate::models::GetLocationResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/locations/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn list_locations(&self, name: Option<&str>) -> Result<crate::models::ListLocationsResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/locations", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}

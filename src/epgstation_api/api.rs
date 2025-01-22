use super::model::{
    FetchEncodeResponse, FetchRecordedProperty, FetchRulesResponse, Record,
    RecordedEndpointResponse, RequestEncodeProperty, Rule, RuleFetchType,
};
use anyhow::{Ok, Result};
use reqwest::Url;

pub struct Client {
    host: Url,
}

impl Client {
    pub fn new(base_uri: Url) -> Self {
        Client { host: base_uri }
    }

    pub async fn fetch_recorded(&self, property: FetchRecordedProperty) -> Result<Vec<Record>> {
        let mut url = self.host.clone();
        url.set_path("/api/recorded");

        url.query_pairs_mut()
            .clear()
            .extend_pairs(&property.to_parameter());

        let response: RecordedEndpointResponse =
            reqwest::get(url).await?.error_for_status()?.json().await?;

        Ok(response.records)
    }

    pub async fn request_encode(&self, encode_property: RequestEncodeProperty) -> Result<()> {
        let mut url = self.host.clone();
        url.set_path("/api/encode");
        let _json = reqwest::Client::new()
            .post(url)
            .json(&encode_property)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn fetch_encode(&self, is_half_width: bool) -> Result<FetchEncodeResponse> {
        let mut url = self.host.clone();
        url.set_path("/api/encode");
        url.query_pairs_mut()
            .append_pair("isHalfWidth", &is_half_width.to_string());

        let response = reqwest::get(url).await?.error_for_status()?.json().await?;

        Ok(response)
    }

    pub async fn fetch_rules(
        &self,
        offset: i64,
        limit: i64,
        type_: Option<RuleFetchType>,
        keyword: Option<String>,
    ) -> Result<Vec<Rule>> {
        let mut url = self.host.clone();
        url.set_path("/api/rules");
        url.query_pairs_mut()
            .append_pair("offset", &offset.to_string())
            .append_pair("limit", &limit.to_string());

        if type_.is_some() {
            url.query_pairs_mut()
                .append_pair("type", &type_.unwrap().to_string());
        }

        if keyword.is_some() {
            url.query_pairs_mut()
                .append_pair("keyword", &keyword.unwrap());
        }

        let response: FetchRulesResponse =
            reqwest::get(url).await?.error_for_status()?.json().await?;

        Ok(response.rules)
    }
}

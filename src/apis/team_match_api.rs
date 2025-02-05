/*
 * Statbotics REST API
 *
 * The REST API for Statbotics. Please be nice to our servers! If you are looking to do large-scale data science projects, use the CSV exports on the GitHub repo.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`read_team_match_v3_team_match_team_match_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadTeamMatchV3TeamMatchTeamMatchGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_team_matches_v3_team_matches_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadTeamMatchesV3TeamMatchesGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Returns a single Team Match object. Requires a team number and match key, e.g. `5511` and `2019ncwak_f1m1`.
pub async fn read_team_match_v3_team_match_team_match_get(configuration: &configuration::Configuration, team: i32, r#match: &str) -> Result<serde_json::Value, Error<ReadTeamMatchV3TeamMatchTeamMatchGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_team = team;
    let p_match = r#match;

    let uri_str = format!("{}/v3/team_match/{team}/{match}", configuration.base_path, team=p_team, match=crate::apis::urlencode(p_match));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReadTeamMatchV3TeamMatchTeamMatchGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns up to 1000 team matches at a time. Specify limit and offset to page through results.
pub async fn read_team_matches_v3_team_matches_get(configuration: &configuration::Configuration, team: Option<i32>, year: Option<i32>, event: Option<&str>, week: Option<i32>, r#match: Option<&str>, elim: Option<bool>, metric: Option<&str>, ascending: Option<bool>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<serde_json::Value>, Error<ReadTeamMatchesV3TeamMatchesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_team = team;
    let p_year = year;
    let p_event = event;
    let p_week = week;
    let p_match = r#match;
    let p_elim = elim;
    let p_metric = metric;
    let p_ascending = ascending;
    let p_limit = limit;
    let p_offset = offset;

    let uri_str = format!("{}/v3/team_matches", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_team {
        req_builder = req_builder.query(&[("team", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_year {
        req_builder = req_builder.query(&[("year", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_event {
        req_builder = req_builder.query(&[("event", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_week {
        req_builder = req_builder.query(&[("week", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_match {
        req_builder = req_builder.query(&[("match", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_elim {
        req_builder = req_builder.query(&[("elim", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_metric {
        req_builder = req_builder.query(&[("metric", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ascending {
        req_builder = req_builder.query(&[("ascending", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ReadTeamMatchesV3TeamMatchesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}


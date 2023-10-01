use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use log::{debug, error};
use reqwest::Client;
use worker::{Env, Result};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/lil_nouns_schema.graphql",
  query_path = "graphql/queries/lil_nouns_query.graphql",
  response_derives = "Clone",
  deprecated = "warn"
)]
pub(crate) struct LilNounsProposalQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/prop_lot_schema.graphql",
  query_path = "graphql/queries/prop_lot_query.graphql",
  response_derives = "Clone",
  deprecated = "warn"
)]
pub(crate) struct PropLotIdeaQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "graphql/schemas/nouns_schema.graphql",
  query_path = "graphql/queries/nouns_query.graphql",
  response_derives = "Clone",
  deprecated = "warn"
)]
pub(crate) struct NounsProposalQuery;

type BigInt = String;
type Date = String;

async fn fetch<QueryType: GraphQLQuery>(
  graphql_url: String,
  variables: <QueryType as GraphQLQuery>::Variables,
) -> Option<<QueryType as GraphQLQuery>::ResponseData> {
  let client = Client::builder()
    .build()
    .map_err(|e| {
      error!("Failed to create client: {}", e);
      debug!("Error details: {:?}", e);
    })
    .ok()?;

  post_graphql::<QueryType, _>(&client, &graphql_url, variables)
    .await
    .map_err(|e| {
      error!("Failed to execute GraphQL request: {}", e);
      debug!("Failure details: {:?}", e);
    })
    .ok()
    .and_then(|response| response.data)
}

pub async fn fetch_lil_nouns_data(env: &Env, id: u64) -> Result<(String, String, String)> {
  let graphql_url = env.var("LIL_NOUNS_GRAPHQL_URL")?.to_string();
  let variables = lil_nouns_proposal_query::Variables { id: id.to_string() };

  let response = fetch::<LilNounsProposalQuery>(graphql_url, variables).await;

  let proposal = match response {
    Some(data) => match data.proposal {
      Some(proposal) => proposal,
      None => return Err("Error message".into()),
    },
    None => return Err("Error message".into()),
  };

  let mut title = proposal.title;
  if title.len() > 60 {
    title.truncate(55);
    title.push_str("...");
  }

  let mut description = proposal.description;
  if description.len() > 160 {
    description.truncate(155);
    description.push_str("...");
  }

  let image = "".to_string();

  Ok((title, description, image))
}

pub async fn fetch_prop_lot_data(env: &Env, id: u64) -> Result<(String, String, String)> {
  let graphql_url = env.var("PROP_LOT_GRAPHQL_URL")?.to_string();
  let variables = prop_lot_idea_query::Variables {
    id: id.try_into().unwrap(),
  };

  let response = fetch::<PropLotIdeaQuery>(graphql_url, variables).await;

  let idea = match response {
    Some(data) => match data.get_idea {
      Some(idea) => idea,
      None => return Err("Error message".into()),
    },
    None => return Err("Error message".into()),
  };

  let mut title = idea.title;
  if title.len() > 60 {
    title.truncate(55);
    title.push_str("...");
  }

  let mut description = idea.tldr;
  if description.len() > 160 {
    description.truncate(155);
    description.push_str("...");
  }

  let image = "".to_string();

  Ok((title, description, image))
}

pub async fn fetch_meta_gov_data(env: &Env, id: u64) -> Result<(String, String, String)> {
  let graphql_url = env.var("NOUNS_GRAPHQL_URL")?.to_string();
  let variables = nouns_proposal_query::Variables { id: id.to_string() };

  let response = fetch::<NounsProposalQuery>(graphql_url, variables).await;

  let proposal = match response {
    Some(data) => match data.proposal {
      Some(proposal) => proposal,
      None => return Err("Error message".into()),
    },
    None => return Err("Error message".into()),
  };

  let mut title = proposal.title;
  if title.len() > 60 {
    title.truncate(55);
    title.push_str("...");
  }

  let mut description = proposal.description;
  if description.len() > 160 {
    description.truncate(155);
    description.push_str("...");
  }

  let image = "".to_string();

  Ok((title, description, image))
}

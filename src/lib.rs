use serde::{Deserialize, Serialize};
use sqids::Sqids;
use url::Url;
use worker::{console_debug, event, Context, Env, Request, Response, Result, Router};

use crate::{
  Community::LilNouns,
  Platform::{Ethereum, MetaGov, PropLot},
};

#[derive(Debug, Serialize, Deserialize)]
struct UrlPayload {
  pub url: String,
  pub sqid: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Community {
  LilNouns = 1,
}

#[derive(Debug, PartialEq)]
enum Platform {
  Ethereum = 1,
  PropLot = 2,
  MetaGov = 3,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  let router = Router::new();

  router
    .get("/", |_, _| Response::ok("Hello, World!"))
    .get_async("/:sqid", |_, ctx| async move {
      if let Some(sqid) = ctx.param("sqid") {
        let sqids = Sqids::default();
        let numbers = sqids.decode(&sqid);

        console_debug!("{:?}", numbers)
      }

      Response::error("Bad Request", 400)
    })
    .post_async("/", |mut req, _| async move {
      let sqids = Sqids::default();
      let mut numbers: Vec<u64> = Vec::new();

      if let Ok(payload) = req.json::<UrlPayload>().await {
        let url = Url::parse(&*payload.url).expect("Invalid URL");

        match url.host_str() {
          Some("lilnouns.wtf") | Some("www.lilnouns.wtf") => {
            let segments: Vec<_> = url
              .path_segments()
              .expect("Cannot get path segments")
              .filter(|segment| !segment.is_empty())
              .collect();

            if segments.is_empty() || segments[0] != "vote" {
              return Response::error("Bad Request", 400);
            }

            if segments[1] == "nounsdao" {
              numbers.push(Community::LilNouns as u64);
              numbers.push(Platform::MetaGov as u64);
              numbers.push(segments[2].parse::<u32>().unwrap().try_into().unwrap());
            } else {
              numbers.push(Community::LilNouns as u64);
              numbers.push(Platform::Ethereum as u64);
              numbers.push(segments[1].parse::<u32>().unwrap().try_into().unwrap());
            }

            return Response::from_json(&UrlPayload {
              url: url.into(),
              sqid: Some(sqids.encode(&*numbers).unwrap()),
            });
          }
          Some("lilnouns.proplot.wtf") | Some("www.lilnouns.proplot.wtf") => {
            numbers.push(Community::LilNouns as u64);

            let segments: Vec<_> = url
              .path_segments()
              .expect("Cannot get path segments")
              .filter(|segment| !segment.is_empty())
              .collect();

            if segments[0] == "idea" {
              numbers.push(Community::LilNouns as u64);
              numbers.push(Platform::PropLot as u64);
            } else {
              return Response::error("Bad Request", 400);
            }

            return Response::from_json(&UrlPayload {
              url: url.into(),
              sqid: Some(sqids.encode(&*numbers).unwrap()),
            });
          }
          _ => return Response::error("Bad Request", 400),
        }
      } else {
        Response::error("Bad Request", 400).unwrap();
      }

      Response::error("Bad Request", 400)
    })
    .run(req, env)
    .await
}

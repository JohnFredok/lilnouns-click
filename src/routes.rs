use serde::{Deserialize, Serialize};
use sqids::Sqids;
use url::Url;
use worker::{Request, Response, ResponseBody, RouteContext};

use crate::routes::{
  Community::LilNouns,
  Platform::{Ethereum, MetaGov, PropLot},
};

#[derive(Debug, Serialize, Deserialize)]
struct UrlPayload {
  pub url: String,
  pub sqid: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Community {
  LilNouns = 1,
}

#[derive(Debug, PartialEq)]
pub enum Platform {
  Ethereum = 1,
  PropLot = 2,
  MetaGov = 3,
}

pub async fn handle_redirect<D>(_req: Request, ctx: RouteContext<D>) -> worker::Result<Response> {
  if let Some(sqid) = ctx.param("sqid") {
    let sqids = Sqids::default();
    let numbers = sqids.decode(&sqid);

    let community = match numbers[0] {
      1 => Some(LilNouns),
      _ => None,
    };

    let platform = match numbers[1] {
      1 => Some(Ethereum),
      2 => Some(PropLot),
      3 => Some(MetaGov),
      _ => None,
    };

    let url = match (community, platform) {
      (Some(LilNouns), Some(Ethereum)) => {
        format!("{}/{}", "https://lilnouns.wtf/vote", numbers[2])
      }
      (Some(LilNouns), Some(PropLot)) => {
        format!("{}/{}", "https://lilnouns.proplot.wtf/idea", numbers[2])
      }
      (Some(LilNouns), Some(MetaGov)) => {
        format!("{}/{}", "https://lilnouns.wtf/vote/nounsdao", numbers[2])
      }
      _ => String::new(),
    };

    let html_doc = format!(
      "
        <html>
            <head>
              <title>{}</title>
              <meta name=\"description\" content=\"{}\">

              <meta property=\"og:url\" content=\"{}\">
              <meta property=\"og:type\" content=\"website\">
              <meta property=\"og:title\" content=\"{}\">
              <meta property=\"og:description\" content=\"{}\">
              <meta property=\"og:image\" content=\"{}\">

              <meta name=\"twitter:card\" content=\"summary_large_image\">
              <meta property=\"twitter:domain\" content=\"{}\">
              <meta property=\"twitter:url\" content=\"{}\">
              <meta name=\"twitter:title\" content=\"{}\">
              <meta name=\"twitter:description\" content=\"{}\">
              <meta name=\"twitter:image\" content=\"{}\">

              <meta http-equiv=\"refresh\" content=\"5; url={}\" />

            </head>
            <body>
                <h1>Redirecting...</h1>
            </body>
        </html>",
      "Lil Nouns",
      "Lil Nouns are just like Nouns, but Lil!",
      url,
      "Your Site Title",
      "A description of your site.",
      "https://your-site.com/image.png",
      "lilnouns.click",
      url,
      "Your Site Title",
      "A description of your site.",
      "https://your-site.com/image.png",
      url,
    );

    return Response::from_body(ResponseBody::Body(html_doc.as_bytes().to_vec()));
  }

  Response::error("Bad Request", 400)
}

pub async fn handle_creation<D>(
  mut req: Request,
  _ctx: RouteContext<D>,
) -> worker::Result<Response> {
  let sqids = Sqids::default();
  let mut numbers: Vec<u64> = Vec::new();

  if let Ok(payload) = req.json::<UrlPayload>().await {
    let url = Url::parse(&*payload.url).expect("Invalid URL");

    return match url.host_str() {
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
          numbers.push(LilNouns as u64);
          numbers.push(MetaGov as u64);
          numbers.push(segments[2].parse::<u32>().unwrap().try_into().unwrap());
        } else {
          numbers.push(LilNouns as u64);
          numbers.push(Ethereum as u64);
          numbers.push(segments[1].parse::<u32>().unwrap().try_into().unwrap());
        }

        Response::from_json(&UrlPayload {
          url: url.into(),
          sqid: Some(sqids.encode(&*numbers).unwrap()),
        })
      }
      Some("lilnouns.proplot.wtf") | Some("www.lilnouns.proplot.wtf") => {
        numbers.push(LilNouns as u64);

        let segments: Vec<_> = url
          .path_segments()
          .expect("Cannot get path segments")
          .filter(|segment| !segment.is_empty())
          .collect();

        if segments[0] == "idea" {
          numbers.push(LilNouns as u64);
          numbers.push(PropLot as u64);
        } else {
          return Response::error("Bad Request", 400);
        }

        Response::from_json(&UrlPayload {
          url: url.into(),
          sqid: Some(sqids.encode(&*numbers).unwrap()),
        })
      }
      _ => Response::error("Bad Request", 400),
    };
  }

  Response::error("Bad Request", 400)
}

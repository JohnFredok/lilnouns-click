use html_minifier::minify;
use serde::{Deserialize, Serialize};
use sqids::Sqids;
use url::Url;
use worker::{Request, Response, ResponseBody, RouteContext};

use crate::{
  queries::{fetch_lil_nouns_data, fetch_meta_gov_data, fetch_prop_lot_data},
  routes::{
    Community::LilNouns,
    Platform::{Ethereum, MetaGov, PropLot},
  },
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

    let (url, title, description, image) = match (community, platform) {
      (Some(LilNouns), Some(Ethereum)) => {
        let url = format!("{}/{}", "https://lilnouns.wtf/vote", numbers[2]);
        let (title, description, image) = fetch_lil_nouns_data(&ctx.env, numbers[2]).await?;
        (url, title, description, image)
      }
      (Some(LilNouns), Some(PropLot)) => {
        let url = format!("{}/{}", "https://lilnouns.proplot.wtf/idea", numbers[2]);
        let (title, description, image) = fetch_prop_lot_data(&ctx.env, numbers[2]).await?;
        (url, title, description, image)
      }
      (Some(LilNouns), Some(MetaGov)) => {
        let url = format!("{}/{}", "https://lilnouns.wtf/vote/nounsdao", numbers[2]);
        let (title, description, image) = fetch_meta_gov_data(&ctx.env, numbers[2]).await?;
        (url, title, description, image)
      }
      _ => (String::new(), String::new(), String::new(), String::new()),
    };

    let html_doc = format!(
      r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">

            <meta property="og:site_name" content="Lil Nouns">
            <meta property="og:url" content="{}">
            <meta property="og:type" content="website">
            <meta property="og:title" content="{}">
            <meta property="og:description" content="{}">

            <meta property="og:image" content="{}">
            <meta property="og:image:secure_url" content="{}" />
            <meta property="og:image:type" content="image/png" />
            <meta property="og:image:width" content="1200" />
            <meta property="og:image:height" content="630" />
            <meta property="og:image:alt" content="{}" />

            <meta name="twitter:card" content="summary_large_image">
            <meta property="twitter:domain" content="lilnouns.click">
            <meta property="twitter:url" content="{}">
            <meta name="twitter:title" content="{}">
            <meta name="twitter:description" content="{}">
            <meta name="twitter:image" content="{}">

            <meta http-equiv="refresh" content="3; url={}" />

            <title>{}</title>
            <meta name="description" content="{}">

            <link rel="preconnect" href="https://fonts.googleapis.com">
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
            <link href="https://fonts.googleapis.com/css2?family=Londrina+Solid:wght@100;300;400;900&display=swap" rel="stylesheet">
        </head>
        <body style="margin: 0; padding: 0; display: flex; justify-content: center; align-items: center; height: 100vh; background-color: #f0f0f0; font-family: 'Londrina Solid', cursive;">
            <div style="text-align: center;">
                <div style="padding: 20px;">
                    <img src="https://lilnouns.wtf/static/media/lil-loading-skull.b7a846e1.gif" alt="Loading Skull" style="width: 192px; height: 192px;">
                    <p style="margin-top: 10px; font-size: 24px; font-weight: bold;">Redirecting...</p>
                    <p><a style="font-size: 24px; text-decoration: none;" href="{}">{}</a></p>
                </div>
            </div>
        </body>
        </html>
    "#,
      url,
      html_escape::encode_text(&title),
      html_escape::encode_text(&description),
      image,
      image,
      html_escape::encode_text(&title),
      url,
      html_escape::encode_text(&title),
      html_escape::encode_text(&description),
      image,
      url,
      html_escape::encode_text(&title),
      html_escape::encode_text(&description),
      url,
      html_escape::encode_text(&title),
    );

    let minified_html = minify(html_doc).expect("Failed to minify HTML");

    let response = Response::from_body(ResponseBody::Body(minified_html.as_bytes().to_vec()));

    return match response {
      Ok(mut res) => {
        res.headers_mut().set("Content-Type", "text/html").unwrap();
        return Ok(res);
      }
      Err(e) => Err(e),
    };
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
          numbers.push(PropLot as u64);
          numbers.push(segments[1].parse::<u32>().unwrap().try_into().unwrap());
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

use url::Url;
use worker::{event, Context, Env, Request, Response, Result, Router};

mod queries;
mod routes;
mod utils;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  let router = Router::new();

  router
    .get("/", |_, _| {
      Response::redirect(Url::parse("https://lilnouns.wtf")?)
    })
    .get_async("/:sqid", routes::handle_redirect)
    .post_async("/", routes::handle_creation)
    .run(req, env)
    .await
}

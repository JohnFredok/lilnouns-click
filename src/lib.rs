use sqids::Sqids;
use worker::{console_log, event, Context, Env, Request, Response, Result, Router};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello, World!"))
        .get_async("/:sqid", |_, ctx| async move {
            if let Some(sqid) = ctx.param("sqid") {
                let sqids = Sqids::default();
                let numbers = sqids.decode(&sqid);

                console_log!("{:?}", numbers)
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}

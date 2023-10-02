# Nounish Pro

## Wrangler

Wrangler is used to develop, deploy, and configure your Worker via CLI.

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

## Usage

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```sh
# run your Worker in an ideal development workflow (with a local server, file watcher & more)
$ pnpm run dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ pnpm run deploy
```

Read the latest `worker` crate documentation here: https://docs.rs/worker

## WebAssembly

All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.

## Issues

If you have any problems, please open an issue on the project issue tracker on the [this repository](https://github.com/nekofar/nounish-pro).

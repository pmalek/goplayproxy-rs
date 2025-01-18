# goplayproxy-rs

This project is a simple [Go Playground][goplay] proxy written in [Rust].

It uses Cloudflare's [workers-rs][cf_workers_repo] ([more info][cf_workers_docs]).

For more technical details on Go Playground, see [this blog post by the Go team][goplay_blog].

[rust]: https://www.rust-lang.org/
[goplay]: https://go.dev/play/
[goplay_blog]: https://go.dev/blog/playground
[cf_workers_docs]: https://developers.cloudflare.com/workers/languages/rust
[cf_workers_repo]: https://github.com/cloudflare/workers-rs

## How to deploy

```
make deploy
```

## How to run

### Locally

```
make run
```

# goplayproxy-rs

This project is a simple [Go Playground][goplay] proxy written in Rust.

It uses [Cloudflare Rust Workers][cf_workers] to proxy requests to the Go Playground.

For more technical details on Go Playground, see [this blog post by the Go team][goplay_blog].

[goplay]: https://go.dev/play/
[goplay_blog]: https://go.dev/blog/playground
[cf_workers]: https://developers.cloudflare.com/workers/languages/rust

## How to deploy

```
make deploy
```

## How to run

### Locally

```
make run
```

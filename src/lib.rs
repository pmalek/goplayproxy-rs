use lazy_static::lazy_static;
use log::*;
use regex::Regex;
use worker::*;

const BASE_URL: &str = "https://go.dev";
const BLOG_DOMAIN: &str = "https://blog.pmalek.dev";
const HUGO_LOCAL: &str = "http://localhost:1313";

lazy_static! {
    static ref RE_BLOG_DOMAIN: Regex =
        Regex::new(r"https://.*--blog-malek-dev\.netlify\.app$").unwrap();
}

#[event(start)]
fn start() {
    console_log!("Worker started successfully!");
    // env_logger::init();
    // env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    info!("XX Worker started successfully!");
}

const HEADER_NAME_CONTENT_TYPE: &str = "Content-Type";
const HEADER_NAME_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";
const HEADER_NAME_ORIGIN: &str = "Origin";

#[event(fetch, respond_with_errors)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        .get_async("/", handler)
        .head_async("/", handler)
        .run(req, env)
        .await
}

async fn handler(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let header_origin = req.headers().get(HEADER_NAME_ORIGIN).unwrap();
    match header_origin {
        Some(_) => console_log!("Origin header: {:?}", header_origin),
        None => {
            console_log!("Missing origin header in request: {:?}", req);
            return Response::error("Bad Request", 400);
            // return Ok(Response::builder().with_status(400).empty())
        } // None => return Err("Origin header is missing".into()),
    }

    // Extract path and create target URL
    let url = format!("{}{}", BASE_URL, req.path());

    // Get content type from request headers
    let content_type = req
        .headers()
        .get(HEADER_NAME_CONTENT_TYPE)?
        .unwrap_or_else(|| "application/octet-stream".into());

    // Log request details
    console_log!("Request URL: {}", url);
    console_log!("Content-Type: {}", content_type);

    // Get request body
    let body = req.bytes().await?;
    console_log!("Request body length: {}", body.len());
    console_log!("Request headers: {:?}", req.headers());

    // Create fetch options
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_body(Some(body.into()))
        .with_headers(Headers::from_iter(vec![(
            HEADER_NAME_CONTENT_TYPE,
            content_type.as_str(),
        )]));

    // Forward the request
    let fetch_request = Request::new_with_init(&url, &init)?;
    let mut response = Fetch::Request(fetch_request).send().await?;

    // Get response body
    let body = response.bytes().await?;

    let response_headers = response.headers().clone();
    // Log response details
    console_log!("Response status: {}", response.status_code());
    console_log!("Response headers: {:?}", response_headers);

    // Create new response with the body and set the status
    let new_response = Response::from_bytes(body)?.with_status(response.status_code());

    // Add CORS header and copy other headers
    let mut headers = response.headers().clone();

    match header_origin {
        Some(origin) => {
            if origin == BLOG_DOMAIN
                || origin == HUGO_LOCAL
                || RE_BLOG_DOMAIN.is_match(origin.as_str())
            {
                headers.set(HEADER_NAME_ALLOW_ORIGIN, origin.as_str())?;
            } else {
                // TODO: maybe return an error?
            }
        }
        _ => {
            // TODO: maybe return an error?
        }
    }

    // Copy headers from the proxied response
    for (key, value) in response_headers.entries() {
        if key != HEADER_NAME_ALLOW_ORIGIN {
            headers.set(&key, &value)?;
        }
    }

    Ok(new_response.with_headers(headers.clone()))
}

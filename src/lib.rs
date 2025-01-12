use worker::*;
use log::info;
use regex::Regex;

const BASE_URL: &str = "https://go.dev";

#[event(start)]
fn start() {
    env_logger::init();
    console_error_panic_hook::set_once();
    // TODO: Logging doesn't work because RUST_LOG is not set
    info!("Worker started successfully!");
}

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Extract path and create target URL
    let url = format!("{}{}", BASE_URL, req.path());
    
    // Get content type from request headers
    let content_type = req
        .headers()
        .get("Content-Type")?
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
            "Content-Type", content_type.as_str(),
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
    let new_response = Response::from_bytes(body)?
        .with_status(response.status_code());

    // TODO: make this static?
    let re = Regex::new(r"https://.*--blog-malek-dev\.netlify\.app$").unwrap();
    // Add CORS header and copy other headers
    let mut headers = response.headers().clone();

    match req.headers().get("origin") {
        Ok(Some(origin)) => {
            if origin == "https://blog.pmalek.dev" ||
               origin == "http://localhost:1313" ||
               re.is_match(origin.as_str()) {
                headers.set("Access-Control-Allow-Origin", origin.as_str())?;
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
        if key != "Access-Control-Allow-Origin" {
            headers.set(&key, &value)?;
        }
    }

    Ok(new_response.with_headers(headers.clone()))
}

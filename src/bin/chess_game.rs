use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

use std::fs;

fn main() { }

#[http_component]
fn ai_home(_req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(fs::read_to_string("/index.html")?.as_str())
        .build())
}
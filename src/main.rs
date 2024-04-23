mod entry;

use std::collections::HashMap;
use std::env;

use axum::extract::Query;
use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;

use crate::entry::get_search;

async fn index_html() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn opensearch_xml() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());
    (headers, include_str!("opensearch.xml"))
}

fn valid_shebang(search: &str) -> Option<(&str, &str)> {
    if let Some(stripped) = search.strip_prefix('!') {
        let (p, s) = stripped.split_once(' ')?;
        let prefix = get_search(p)?;
        Some((prefix, s))
    } else {
        None
    }
}

fn get_search_url(search: Option<&String>, use_duckduckgo: bool) -> String {
    let default_search = if use_duckduckgo {
	"https://duckduckgo.com/?q={}"
    } else {
	"https://google.com/search?q={}"
    };
    match search {
        Some(search) => match valid_shebang(search) {
            Some((p, s)) => p.replace("{}", s),
            None => default_search.replace("{}", search),
        },
        None => String::from("/index.html"),
    }
}

async fn search(
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let use_duckduckgo = if let Some(cookie) = headers.get(header::COOKIE) {
	cookie.to_str().unwrap().contains("DuckDuckGo")
    } else {
	false
    };
    Redirect::to(&get_search_url(params.get("q"), use_duckduckgo))
}

async fn set_engine(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    if let Some(engine) = params.get("search-engine") {
        if let Ok(hv) = HeaderValue::from_str(engine) {
            headers.insert(header::SET_COOKIE, hv);
        }
    }
    headers
}

#[tokio::main]
async fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is bad"),
        Err(_) => 3000,
    };

    let app = Router::new()
        .route("/index.html", get(index_html))
        .route("/opensearch.xml", get(opensearch_xml))
        .route("/search", get(search))
        .route("/setengine", get(set_engine));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Starting!");
    axum::serve(listener, app).await.unwrap();
}

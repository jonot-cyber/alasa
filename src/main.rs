mod entry;

use std::{collections::HashMap, net::Ipv4Addr, str::FromStr};
use std::env;

use warp::{http::Uri, reply::Reply, Filter};

use crate::entry::get_search;

fn create_uri(search: &str) -> Option<Uri> {
    let uri = if search.starts_with('!') {
	let search = &search[1..];
	let (shebang, search_term) = search.split_once(' ')?;
	let search_str = get_search(shebang)?;
	search_str.replace("{}", &search_term.replace(' ', "+"))
    } else {
	format!("https://google.com/search?q={}", &search.replace(' ', "+"))
    };
    Uri::from_str(&uri).ok()
}

#[tokio::main]
async fn main() {
    let root_path = warp::path("search")
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("q") {
	    Some(name) => {
		match create_uri(name) {
		    Some(uri) => warp::redirect::found(uri).into_response(),
		    None => "Couldn't create a URL".into_response(),
		}
	    },
	    None => {
		warp::redirect::found(Uri::from_static("/index.html")).into_response()
	    },
	});

    let index = warp::path("index.html")
        .map(|| {
	    warp::reply::html(include_str!("index.html"))
	});
    
    let opensearch = warp::path("opensearch.xml")
        .map(|| {
	    include_str!("opensearch.xml")
	});

    let routes = warp::get()
        .and(root_path
            .or(index)
            .or(opensearch));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is bad"),
        Err(_) => 3000,
    };

    println!("Starting!");
    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}

mod entry;

use std::{collections::HashMap, net::Ipv4Addr, str::FromStr};

use warp::{http::Uri, reply::Reply, Filter};

use crate::entry::get_search;

fn create_uri(search: &str) -> Option<Uri> {
    if !search.starts_with('!') {
	return None;
    }
    let search = &search[1..];
    let (shebang, search_term) = search.split_once(' ')?;
    let search_str = get_search(shebang)?;
    let uri = search_str.replace("{}", &search_term.replace(' ', "+"));
    Uri::from_str(&uri).ok()
}

#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("q") {
	    Some(name) => {
		match create_uri(name) {
		    Some(uri) => warp::redirect::found(uri).into_response(),
		    None => "Couldn't create a URL".into_response(),
		}
	    },
	    None => "Bad".into_response(),
	});

    println!("Starting!");
    warp::serve(example1).run((Ipv4Addr::LOCALHOST, 8080)).await
}

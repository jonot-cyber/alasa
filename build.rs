use std::{env, fs::File, io::BufWriter, io::Write, path::Path};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    write!(
        &mut file,
        "static ENTRIES: phf::Map<&'static str, &'static str> = {}",
        phf_codegen::Map::new()
            .entry("g", "\"https://google.com/search?q={}\"")
            .entry("y", "\"https://youtube.com/results?search_query={}\"")
	    .entry("w", "\"https://en.wikipedia.org/wiki/Special:Search?search={}\"")
	    .entry("mc", "\"https://minecraft.wiki/w/Special:Search?search={}\"")
	    .entry("rs", "\"https://lib.rs/search?q={}\"")
	    .entry("aw", "\"https://wiki.archlinux.org/index.php?title=Special%3ASearch&search={}\"")
	    .entry("gh", "\"https://github.com/search?q={}&type=repositories\"")
	    .entry("deb", "\"https://packages.debian.org/search?keywords={}&searchon=names&suite=stable&section=all\"")
            .build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();
}

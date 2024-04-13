include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn get_search(shebang: &str) -> Option<&'static str> {
    ENTRIES.get(shebang).cloned()
}

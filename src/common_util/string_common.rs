

pub fn string_to_static_str(s: &str) -> &'static str {
    Box::leak(String::from(s).into_boxed_str())
}
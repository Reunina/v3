#![allow(unused)]
/// return a list of 5 languages to learn
pub fn languages() -> Vec<String> {
    vec![
        "Rust".to_string(),
        "JavaScript".to_string(),
        "Elixir".to_string(),
        "Lua".to_string(),
        "Scheme".to_string(),
    ]
}
/// Grab a language from your top five list
pub fn grab(which: usize) -> Option<String> {
    if let Some(lang) = languages().get(which) {
        Some(lang.into())
    } else {
        None
    }
}
/// Gather your top languages to learn
pub fn humans_and_computers(human: Vec<String>) -> Vec<String> {
    let mut list = languages();
    list.extend(human);
    list
}

/// Create a new list and add one language
pub fn add_one(lang: String) -> Vec<String> {
    let mut list = vec![];
    list.push(lang);
    list.to_vec()
}

/// Remove and return the last language from the provided list
pub fn remove_last_one(list: &mut Vec<String>) -> Option<String> {
    list.pop()
}

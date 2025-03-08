pub fn gen_words() -> Vec<String> {
    include_str!("words.txt")
        .trim()
        .to_lowercase()
        .split("\n")
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect()
}

pub fn filter_down(list: &Vec<String>, head: impl AsRef<str>) -> Vec<String> {
    list.iter()
        .filter(|w| w.starts_with(head.as_ref()))
        .cloned()
        .collect()
}
use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct BookmarkList(pub BTreeMap<String, String>);

impl BookmarkList {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("\x1B[1m{}\x1B[0m \x1B[38;5;249m{}\x1B[0m", key, value))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn command_at(&self, index: usize) -> String {
        self.0.values().cloned().collect::<Vec<String>>()[index].clone()
    }

    pub fn has_item<T: AsRef<str>>(&self, name: T) -> bool {
        self.0.contains_key(name.as_ref())
    }

    pub fn get_item<T: AsRef<str>>(&self, name: T) -> &String {
        self.0.get(name.as_ref()).unwrap()
    }
}

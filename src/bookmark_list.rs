use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct BookmarkList(pub BTreeMap<String, String>);

impl BookmarkList {
    pub fn to_skim_input(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("{} \x1B[38;5;249m({})\x1B[0m", key, value))
            .collect::<Vec<String>>()
            .join("\n")
    }

    // TODO: think about reducing cloning; also raw index access is bad
    pub fn command_at(&self, index: usize) -> String {
        self.0.values().cloned().collect::<Vec<String>>()[index].clone()
    }
}

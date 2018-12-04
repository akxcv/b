mod bookmark_list;
mod command;

use crate::bookmark_list::BookmarkList;
use skim::{Skim, SkimOptions};
use std::default::Default;
use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use std::path::Path;

fn read_config_file() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let path = Path::new(&home_dir).join(".b");
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let input = read_config_file();
    let list: BookmarkList = serde_yaml::from_str(&input).unwrap();
    let skim_input = list.to_skim_input();

    let options: SkimOptions = SkimOptions::default().ansi(true);
    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(skim_input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    // TODO quit unless any items are present

    let item = &selected_items[0];
    let index = item.get_index();
    let cmd = list.command_at(index);

    command::run_command(cmd);
}

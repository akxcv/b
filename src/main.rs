mod bookmark_list;
mod command;

use crate::bookmark_list::BookmarkList;
use clap::clap_app;
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

fn run_skim(list: &BookmarkList, options: SkimOptions) {
    let skim_input = list.to_skim_input();

    match Skim::run_with(&options, Some(Box::new(Cursor::new(skim_input))))
        .map(|out| out.selected_items)
    {
        Some(selected_items) => {
            let item = &selected_items[0];
            let index = item.get_index();
            let cmd = list.command_at(index);

            command::run_command(cmd);
        }
        None => {},
    }
}

fn main() {
    let input = read_config_file();
    let list: BookmarkList = serde_yaml::from_str(&input).unwrap();

    let matches = clap_app!(app =>
        (name: "b")
        (@arg QUERY: "bookmark name")
    )
    .get_matches();

    match matches.value_of("QUERY") {
        Some(query) => {
            if list.has_item(query) {
                command::run_command(list.get_item(query));
            } else {
                run_skim(&list, SkimOptions::default().query(query).ansi(true));
            }
        }
        None => run_skim(&list, SkimOptions::default().ansi(true)),
    };
}

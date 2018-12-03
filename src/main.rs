use skim::{Skim, SkimOptions};
use std::collections::BTreeMap;
use std::default::Default;
use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let options: SkimOptions = SkimOptions::default().ansi(true);

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

    let map: BTreeMap<String, String> = serde_yaml::from_str(&input).unwrap();
    let skim_input = map
        .iter()
        .map(|(key, value)| format!("{} \x1B[38;5;249m({})\x1B[0m", key, value))
        .collect::<Vec<String>>()
        .join("\n");

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(skim_input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let item = &selected_items[0];
    let index = item.get_index();
    let cmd = &map.values().cloned().collect::<Vec<String>>()[index];

    let cmd_args = &cmd.split(' ').collect::<Vec<&str>>();
    let first_non_env_var_idx = fnev(cmd_args);

    if let Some(x) = first_non_env_var_idx {
        let out = Command::new("command")
            .args(&["-v", cmd_args[x]])
            .output()
            .unwrap();
        let out_str = String::from_utf8_lossy(&out.stdout);

        if out_str.len() > 0 {
            let mut c = Command::new(cmd_args[x]);
            &cmd_args[0..x].iter().for_each(|v| {
                let s = v.split('=').collect::<Vec<&str>>();
                c.env(s[0], s[1]);
            });

            c.args(&cmd_args[x+1..])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit());
            let mut c = c.spawn().unwrap();
            c.wait().unwrap();
        } else {
            println!("{}", cmd);
        }
    } else {
        println!("{}", cmd);
    }
}

fn fnev(args: &Vec<&str>) -> Option<usize> {
    args.iter()
        .enumerate()
        .find(|(_i, arg)| arg.split('=').collect::<Vec<&str>>().len() != 2)
        .map(|(i, _arg)| i)
}

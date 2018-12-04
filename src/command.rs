use std::process::{Command, Stdio};

struct CommandData {
    raw_pieces: Vec<String>,
    pub env: Vec<(String, String)>,
    pub cmd: String,
    pub args: Vec<String>,
}

impl From<String> for CommandData {
    fn from(s: String) -> Self {
        let pieces: Vec<&str> = s.split(' ').collect();
        if let Some(cmd_index) = get_cmd_index(&pieces) {
            Self {
                raw_pieces: pieces.iter().map(|x| String::from(*x)).collect(),
                env: pieces[0..cmd_index]
                    .iter()
                    .map(|piece| {
                        let split: Vec<&str> = piece.split('=').collect();
                        (String::from(split[0]), String::from(split[1]))
                    })
                    .collect(),
                cmd: String::from(pieces[cmd_index]),
                args: pieces[cmd_index + 1..]
                    .iter()
                    .map(|piece| String::from(*piece))
                    .collect(),
            }
        } else {
            Self {
                raw_pieces: pieces.iter().map(|x| String::from(*x)).collect(),
                env: vec![],
                cmd: String::from(pieces[0]),
                args: pieces[1..]
                    .iter()
                    .map(|piece| String::from(*piece))
                    .collect(),
            }
        }
    }
}

impl CommandData {
    pub fn to_command(&self) -> Command {
        let mut command = shell::cmd!(&self.cmd).command;

        self.env.iter().for_each(|(k, v)| {
            command.env(k, v);
        });

        command.args(&self.args);

        command.stdin(Stdio::inherit()).stdout(Stdio::inherit());
        let cmd = command;
        cmd
    }

    // NOTE: expanding env vars via google/rust-shell. Using a dirty hack to piece it together
    // from a `Command`'s Debug impl
    pub fn to_string(&self) -> String {
        let cmd_string = self.raw_pieces.join(" ");
        let c = shell::cmd!(&cmd_string).command;
        let x = format!("{:?}", c);
        let x = x.split("\" \"").collect::<Vec<&str>>().join(" ");
        format!("{}", x[1..x.len() - 1].to_string())
    }
}

fn get_cmd_index(pieces: &Vec<&str>) -> Option<usize> {
    pieces
        .iter()
        .enumerate()
        .find(|(_i, arg)| arg.split('=').collect::<Vec<&str>>().len() != 2)
        .map(|(i, _arg)| i)
}

fn is_runnable_command(command: &str) -> bool {
    let out = Command::new("command")
        .args(&["-v", command])
        .output()
        .unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    out_str.len() > 0
}

pub fn run_command<T: AsRef<str>>(cmd: T) {
    let data = CommandData::from(cmd.as_ref().to_owned());

    if is_runnable_command(&data.cmd) {
        data.to_command().spawn().unwrap().wait().unwrap();
    } else {
        println!("{}", data.to_string());
    }
}

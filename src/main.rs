use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    time::SystemTime,
};

use chrono::{DateTime, Local, NaiveDateTime, NaiveTime};
use clap::Parser;

#[derive(clap::Parser)]
pub struct Cli {
    /// Add new pill. Format: pill-name:hh:mm
    #[arg(long)]
    add: Option<String>,

    /// Update specified pill
    #[arg(short, long)]
    update: Option<String>,
}

fn main() {
    let config_dir = dirs::config_dir().expect("Your os is stupid bitchass");
    let pill_dir = config_dir.join("pill-reminder");

    fs::create_dir_all(&pill_dir).expect("Fuckass can't make files on your system bitch");
    let conf_path = pill_dir.join("config");
    _ = fs::File::create_new(&conf_path);
    let mut config_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open(conf_path)
        .expect("Fuckass can't open the file bitch");

    let cli = Cli::parse();

    if let Some(add) = cli.add {
        add_entry(&mut config_file, &add);
        return;
    }

    if let Some(upd) = cli.update {
        update_pill(&upd);
        return;
    }

    let mut file_contents = String::new();
    config_file
        .read_to_string(&mut file_contents)
        .expect("Couldn't read file, bitch");

    let lines = file_contents.split('\n');

    let datetime_now: DateTime<Local> = DateTime::from(SystemTime::now());
    let now = datetime_now.naive_local();

    for line in lines {
        if let Some(name) = check_line(line, now) {
            println!("{name} is due");
        }
    }
}

pub fn add_entry(file: &mut fs::File, s: &str) {
    file.write_all(s.as_bytes())
        .expect("Couldn't write to the file bitchass");
    file.write_all(b"\n")
        .expect("Couldn't write to the file bitchass");
}

pub fn check_line(line: &str, now: NaiveDateTime) -> Option<String> {
    let (lhs, rhs) = if let Some(a) = line_split(line) {
        a
    } else {
        return None;
    };

    let name = lhs.to_string();

    if let Ok(time) = NaiveTime::parse_from_str(rhs, "%H:%M") {
        let time = time;

        if check_pill_date(&name, now) && now.time() >= time {
            Some(name)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn check_pill_date(pill: &str, now: NaiveDateTime) -> bool {
    let data_path = dirs::data_dir().unwrap().join("pill-reminder");

    let file = data_path.join(pill);

    if let Ok(a) = fs::File::open(file) {
        let modified = a.metadata().unwrap().modified().unwrap();

        let localtime: chrono::DateTime<Local> = modified.into();
        let datetime = localtime.naive_local();

        println!("{}, {}", now.date(), datetime.date());
        if now.date() <= datetime.date() {
            return false;
        }
    }

    true
}

pub fn line_split(line: &str) -> Option<(&str, &str)> {
    for (i, ch) in line.chars().enumerate() {
        if ch == ':' {
            return Some((&line[0..i], &line[(i + 1)..]));
        }
    }
    None
}

pub fn update_pill(pill: &str) {
    let data = dirs::data_dir().unwrap().join("pill-reminder");
    fs::create_dir_all(&data).unwrap();
    let filename = data.join(pill);

    touch(&filename);
}

fn touch(path: &PathBuf) {
    fs::remove_file(path).unwrap();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap()
        .write(b"")
        .unwrap();
}

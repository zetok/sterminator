/*
    Copyright © 2016 Zetok Zalbavar <zexavexxe@gmail.com>

    This file is part of Sterminator.

    Sterminator is libre software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Sterminator is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Sterminator.  If not, see <http://www.gnu.org/licenses/>.
*/



extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::exit;


enum Mode {
    Error,
    Help,
    Replace,
}

fn get_args() -> Vec<String> {
    env::args().collect()
}

fn parse_args(args: &[String]) -> Mode {
    if args.len() == 1 || args.len() > 3 { return Mode::Error }

    // parse help
    if args.len() == 2 {
        return match args[1].as_str() {
            "-h" | "--help" => Mode::Help,
            _ => Mode::Error,
        }
    }

    // just assume that those are files
    Mode::Replace
}

/// print help msg
fn help_msg() {
    println!("Usage:\n\t./sterminator <website> <json>\n");
    println!("Sterminator replaces text enclosed in 3 brackets: \
    `[[[text]]]` with corresponding string that JSON file contains. \
    In case where string provided by JSON key is empty, JSON key is \
    used to replace text between brackets.\
    \n\n\
    E.g.\
    \n\n`website` file contains `<html><body>[[[pls replace]]]</body>\
    </html>`\
    \nand `json` file contains `{{ \"pls replace\": \"replaced\" }}`\
    \n\n\
    Sterminating will result in: `<html><body>replaced</body></html>`");
}

/// print error msg
fn error_msg(){
    println!("Ur doin' something worg.\n");
}

fn open_file(file: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(file)
        .expect(&format!("Failed to open file {}\
            \n\tPerhaps check permissions, etc?\n", file))
}

fn main() {
    let args = get_args();
    match parse_args(&args) {
        Mode::Error => { error_msg(); help_msg(); exit(1) },
        Mode::Help => { help_msg(); exit(0) },
        _ => {},
    }

    // **ASSUMES THAT THERE ARE >= 2 FILES!**
    let mut web_file = open_file(&args[1]);
    let mut json_file = open_file(&args[2]);

    let json = Json::from_reader(&mut json_file)
        .expect(&format!("Failed to get json from {:?}\
            \n\tIs is reaallly JSON?\n", &json_file));

    let mut www = String::new();
    drop(web_file.read_to_string(&mut www));

    for (original, translation) in json.as_object()
        .expect(&format!("JSON {} is not an object!", json))
    {
        // use original if not string
        let tr = translation.as_string().unwrap_or(original);

        // Replaces `[[[String]]]` with supplied `String`.
        //
        // If there is no matching `[[[String]]]`, there's no change.
        if tr.is_empty() {
            www = www.replace(&format!("[[[{}]]]", original), original);
        } else {
            www = www.replace(&format!("[[[{}]]]", original), tr);
        }
    }

    // write the file down
    drop(web_file.set_len(0).expect(
        &format!("Failed to truncate file {:?}", web_file)));
    web_file.write_all(www.as_bytes())
        .expect(&format!("Failed to write to file {:?}", web_file));
    println!("Done: {}", &args[1]);
}

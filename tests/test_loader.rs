#![cfg(test)]

use std::{process::Stdio, str};

use riglet::{FIGfont, FIGure};
use run_figlet::RunFiglet;

fn run_test<S1: AsRef<str>, S2: AsRef<str>>(font: S1, text: S2) {
    let cmd_res = RunFiglet::new()
        .stderr(Stdio::null())
        .arg("-f")
        .arg(font.as_ref())
        .arg(text.as_ref())
        .output();
    assert!(cmd_res.is_ok());
    let cmd_res = cmd_res.unwrap();
    assert!(cmd_res.status.success());
    let cmd_res = str::from_utf8(&cmd_res.stdout[..]);
    assert!(cmd_res.is_ok());
    let cmd_res = cmd_res.unwrap();

    let font = FIGfont::load_from(font.as_ref());
    assert!(font.is_ok());
    let font = font.unwrap();
    let mut figure = FIGure::new(&font, 80);
    assert!(figure.add(text.as_ref()).is_ok());
    let r_res: String = figure.to_string();

    assert_eq!(cmd_res, &r_res);
}

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

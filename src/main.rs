use nix::sched;
use std::error::Error;
use std::fs::File;
use std::os::unix::io::IntoRawFd;
use std::{env, process};

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Not enough argurments!");
        process::exit(1);
    }

    let ns = format!("/proc/{}/ns/mnt", args[1].clone());
    let fd = File::open(ns)?;
    sched::setns(fd.into_raw_fd(), sched::CloneFlags::CLONE_NEWNS).unwrap_or_else(|err| {
        eprintln!("Failed to setns: {}", err);
        process::exit(1);
    });

    let cmd = args[2].clone();
    match cmd.as_str() {
        "cat" => lib::cat(args[3].clone())?,
        "ls" => lib::ls(&args[3..])?,
        "exec" => lib::exec(args[3].clone())?,
        _ => eprintln!("Command not supported."),
    }

    Ok(())
}

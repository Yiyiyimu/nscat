use nix::sched;
use nix::unistd;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::{self, File};
use std::os::unix::io::IntoRawFd;
use std::path::Path;
use std::{env, process};

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
        "cat" => cat(args[3].clone())?,
        "ls" => ls(&args[3..])?,
        "exec" => exec(args[3].clone())?,
        _ => eprintln!("Command not supported."),
    }

    Ok(())
}

fn cat(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    println!("{}", contents);
    Ok(())
}

fn ls(args: &[String]) -> Result<(), Box<dyn Error>> {
    let is_long = args[0] == "-l";
    let path_string = if is_long {
        args[1].clone()
    } else {
        args[0].clone()
    };
    let path = Path::new(&path_string);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap();
        if is_long {
            let meta = entry.metadata()?;
            if meta.file_type().is_symlink() {
                let symlink_file = fs::read_link(entry.path())?;
                println!("{} -> {:?}", name, symlink_file);
            } else {
                println!("{}", name);
            }
        } else {
            print!("{} ", name);
        }
    }
    Ok(())
}

fn exec(cmd_in: String) -> Result<(), Box<dyn Error>> {
    let cmd_cstring = vec![CString::new(cmd_in).unwrap()];
    let cmd: Vec<&CStr> = cmd_cstring.iter().map(CString::as_c_str).collect();
    unistd::execvp(&cmd[0], &cmd)?;
    Ok(())
}

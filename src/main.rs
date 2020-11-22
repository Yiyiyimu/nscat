use nix::sched;
use std::error::Error;
use std::fs::{self, File};
use std::os::unix::io::IntoRawFd;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough argurments!");
        process::exit(1);
    }

    let fd = File::open(args[1].clone()).unwrap_or_else(|err| {
        println!("Failed to open file: {}", err);
        process::exit(1);
    });

    sched::setns(fd.into_raw_fd(), sched::CloneFlags::CLONE_NEWNS).unwrap_or_else(|err| {
        println!("Failed to setns: {}", err);
        process::exit(1);
    });

    //FOR NSENTER
    //use nix::unistd;
    //use std::ffi::{CStr, CString};
    //let cmd_cstring = vec![CString::new(args[2].clone()).unwrap()];
    //let cmd: Vec<&CStr> = cmd_cstring.iter().map(CString::as_c_str).collect();
    //unistd::execvp(&cmd[0], &cmd).expect("execvp failed");
    cat(args[2].clone());

    Ok(())
}

fn cat(filename: String) {
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to print file: {}", err);
        process::exit(1);
    });

    println!("{}", contents);
}

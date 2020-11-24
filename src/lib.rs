use nix::unistd;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;

pub fn cat(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    println!("{}", contents);
    Ok(())
}

pub fn ls(args: &[String]) -> Result<(), Box<dyn Error>> {
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

pub fn exec(cmd_in: String) -> Result<(), Box<dyn Error>> {
    let cmd_cstring = vec![CString::new(cmd_in).unwrap()];
    let cmd: Vec<&CStr> = cmd_cstring.iter().map(CString::as_c_str).collect();
    unistd::execvp(&cmd[0], &cmd)?;
    Ok(())
}

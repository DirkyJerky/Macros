#[macro_use]
extern crate clap;
extern crate simple_error;

use clap::App;
use clap::Arg;
use clap::ArgMatches;

use simple_error::SimpleError;

use std::process::Command;
use std::process::Output;
use std::os::unix::process::CommandExt;
use std::error::Error;
use std::io::Write;
use std::process::exit;
use std::fmt::Display;
use std::fmt;
use std::string::String;
use std::path::Path;

fn main() {
    let args = App::new("Macro runner")
                   .version(crate_version!())
                   .author("Geoff Yoerger <geoffreyiy1@gmail.com> [https://github.com/DirkyJerky]")
                   .about("Runs defined macros from the current active window")
                   .arg(Arg::with_name("id")
                                .index(1)
                                .required(true)
                                .help("Macro identifier, suggest using integers"))
                   .get_matches();
    if let Err(e) = run(args) {
        // On success the current process will be replaced with the script, the only way for
        // execution to make it to this point is if an error happens before that
        writeln!(&mut std::io::stderr(), "{:?}", e);
        exit(1);
    }
    exit(0)
}

fn run(args: ArgMatches) ->  Result<(), Box<Error>> {
    // use `xdotool` to get PID of currently active window
    let Output {status, stdout, stderr, .. } = 
                      Command::new("xdotool")
                              .arg("getactivewindow")
                              .arg("getwindowpid")
                              .env("DISPLAY", ":0")
                              .output()
                              ?;
    if !status.success() {
        println!("{}", String::from_utf8(stdout).unwrap());
        println!("{}", String::from_utf8(stderr).unwrap());
        return Err(Box::new(SimpleError::new(
                status.code().map(|n| format!("xdotool exited with error code {}", n)).unwrap_or("xdotool unsuccessfully completed without error code".to_string())         
            )));
    }
    
    let pid_str = String::from_utf8(stdout)?;

    let exe_file = Path::new("/proc").join(pid_str.trim()).join("exe");

    let exe_path = std::fs::read_link(exe_file)?;

    let window_name = exe_path.file_name().ok_or(SimpleError::new("Executables arent supposed to be non-existant"))?;

    let mut working_dir = std::env::current_exe()?;
    working_dir.set_file_name("by-exe");
    working_dir.push(window_name);

    if !working_dir.is_dir() {
        println!("{}", working_dir.to_str().unwrap());
        return Err(Box::new(SimpleError::new(
                    format!("No macro directory for current active window: '{}'", 
                            window_name.to_str().ok_or(SimpleError::new("Executable name isnt valid UTF-8"))?
                    ))));
    }

    working_dir.push(format!("{}.sh", args.value_of("id").expect("required arg should exist here")));

    if !working_dir.is_file() {
        return Err(Box::new(SimpleError::new(format!("No script: {:?}", working_dir.to_str()))));
    }

    return Err(Box::new(
        Command::new("/bin/bash")
                .arg(working_dir.as_os_str())
                .env("DISPLAY", ":0")
                .exec()
        ));
}

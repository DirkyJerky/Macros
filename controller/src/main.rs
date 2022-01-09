#[macro_use]
extern crate clap;
extern crate simple_error;

use clap::App;
use clap::Arg;
use clap::ArgMatches;

use simple_error::SimpleError;

use std::process::Command;
use std::process::Output;
use std::error::Error;
use std::io::Write;
use std::process::exit;
use std::string::String;
use std::path::Path;
use std::os::unix::process::CommandExt;

fn main() {
    let args = App::new("Macro runner")
                   .version(crate_version!())
                   .author("Geoff Yoerger <geoffreyiy1@gmail.com> [https://github.com/DirkyJerky]")
                   .about("Runs defined macros from the current active window")
                   .arg(Arg::with_name("id")
                                .index(1)
                                .required(true)
                                .help("Macro identifier"))
                   .arg(Arg::with_name("v")
                                .short("v")
                                .multiple(true)
                                .help("Set verbosity level"))
                   .get_matches();

    let (e1, e2) = run(args);

    // On success the current process will be replaced with the script, the only way for
    // execution to make it to this point is if an error happens before that
    writeln!(&mut std::io::stderr(), "{:?}", e1);
    writeln!(&mut std::io::stderr(), "{:?}", e2);
    exit(1);
}

fn get_current_window() -> Result<String, Box<dyn Error>> {
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

    return match exe_path.file_name() {
        Some(name_osstr) => match name_osstr.to_str() {
            Some(name_str) => Ok(name_str.to_string()),
            None => Err(Box::new(SimpleError::new("File not valid UTF-8"))),
        },
        None => Err(Box::new(SimpleError::new("Executable path does not exist"))),
    }
}

fn try_specific_exe(exe_name: &str, script_id: &str) -> Box<dyn Error> {
    let mut working_dir = match std::env::current_exe() {
        Ok(r) => r,
        Err(err) => return Box::new(err),
    };
    working_dir.set_file_name("by-exe");
    working_dir.push(exe_name.clone());

    if !working_dir.is_dir() {
        println!("{}", working_dir.to_str().unwrap());
        return Box::new(SimpleError::new(format!("No macro directory for current active window: '{}'", exe_name)));
    }

    working_dir.push(format!("{}.sh", script_id));

    if !working_dir.is_file() {
        return Box::new(SimpleError::new(format!("No script: {:?}", working_dir.to_str())));
    }


    return Box::new(
            Command::new("/bin/bash")
            .arg(working_dir.as_os_str())
            .env("DISPLAY", ":0")
            .exec()
                   );
}

fn try_default(script_id: &str) -> Box<dyn Error> {
    let mut working_dir = match std::env::current_exe() {
        Ok(r) => r,
        Err(err) => return Box::new(err),
    };
    working_dir.set_file_name("default");
    working_dir.push(format!("{}.sh", script_id));

    if !working_dir.is_file() {
        return Box::new(SimpleError::new(format!("No default script: {:?}", working_dir.to_str())));
    }


    return Box::new(
            Command::new("/bin/bash")
            .arg(working_dir.as_os_str())
            .env("DISPLAY", ":0")
            .exec()
                   );
}

fn run(args: ArgMatches) ->  (Box<dyn Error>, Box<dyn Error>) {
    let exe_name_res = get_current_window();
    let script_id = args.value_of("id").expect("required arg should exist here");

    let specific_exe_attempt = match exe_name_res {
        Ok(exe_name) => try_specific_exe(exe_name.as_str(), script_id),
        Err(err) => err,
    };

    let default_attempt = try_default(script_id);
    
    // Both attempts failed here
    return (specific_exe_attempt, default_attempt);
}


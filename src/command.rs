use std::process::Command;

macro_rules! command_result(
    ($out : expr) => {
        if let Some(code) = $out.status.code() {
            if code == 0 { Ok(()) } else { Err(String::from_utf8($out.stderr).unwrap()) }
        } else {
            Err("process was terminated by a signal".to_string())
        }
    }   
);

pub fn run_command(path: &str, args: &[&str]) -> Result<(), String> {
    match Command::new(path).args(args).output() {
        Ok(output) => command_result!(output),
        Err(e) => Err(e.to_string()),
    } 
}

pub fn build_and_run(dir: &str, lang: &str, b_args: &[&str], r_args: &[&str]) -> Result<(), String> {
        let primary_build_path = &(dir.to_string() + lang + "/build"); 
        let secondary_build_path = &(dir.to_string() + "common/build"); 
        let primary_run_path = &(dir.to_string() + lang + "/run"); 
        let secondary_run_path = &(dir.to_string() + "common/run"); 

        if let Ok(_) = run_command("cp", &[primary_build_path, "."]) {
            try!(run_command("./build", b_args));
            if let Err(_) = run_command("./run", r_args) {
                try!(run_command("cp", &[primary_run_path, "."]));
                try!(run_command("./run", r_args));
            } 
        } else {
            if let Ok(_) = run_command("cp", &[secondary_build_path, "."]) {
                try!(run_command("./build", b_args));
                if let Err(_) = run_command("./run", r_args) {
                    try!(run_command("cp", &[secondary_run_path, "."]));
                    try!(run_command("./run", r_args));
                } 
            } else {
                return Err("no build script found in ".to_string() + dir + " for " + lang);
            }
        }

        run_command("rm", &["-f", "./build", "./run"]).unwrap();
        Ok(())
}

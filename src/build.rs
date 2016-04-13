use std::process::*;

pub struct Builder<'a> {
    code: &'a str,
    lang: &'a str
} 

impl<'a> Builder<'a> {
    pub fn new(code: &'a str, lang: &'a str) -> Builder<'a> {
        Builder { code: code, lang: lang }
    }

    pub fn build(&self) -> Result<(), String> {
        try!(self.build_builder());
        try!(self.run_builder());
        Ok(())
    }

    fn build_builder(&self) -> Result<(), String> {
        let script_path = "conf/build/".to_string() + self.lang + "/build";
        let default_script_path = "conf/build/common/build";

        let output = Command::new(script_path)
                        .output()
                        .unwrap_or(Command::new(default_script_path).output().unwrap());
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8(output.stdout).unwrap())
        }
    }

    fn run_builder(&self) -> Result<(), String> {
        let script_path = "conf/build/".to_string() + self.lang + "/run";
        let default_script_path = "conf/build/common/run";

        let output = Command::new(script_path)
                        .output()
                        .unwrap_or(Command::new(default_script_path).output().unwrap());
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8(output.stdout).unwrap())
        }
    }
}

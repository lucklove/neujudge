use std::fs::File;
use std::io::Write;
use command::build_and_run;

pub struct Builder<'a> {
    code: &'a str,
    lang: &'a str
} 

impl<'a> Builder<'a> {
    pub fn new(code: &'a str, lang: &'a str) -> Builder<'a> {
        Builder { code: code, lang: lang }
    }

    pub fn build(&self, target: &str) -> Result<(), String> {
        if let Ok(mut f) = File::create("solution") {
            if let Err(e) = f.write_all(self.code.as_bytes()) {
                return Err(e.to_string());
            }
        } else {
            return Err("can't create solution file".to_string());
        }

        build_and_run("../conf/build/", self.lang, &[], &[target, "solution"])
    }
}

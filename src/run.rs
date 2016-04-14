use command::build_and_run;

pub struct Runner<'a> {
    prog: &'a str,
    lang: &'a str,
}

impl<'a> Runner<'a> {
    pub fn new(prog: &'a str, lang: &'a str) -> Runner<'a> {
        Runner {
            prog: prog,
            lang: lang,
        }
    }

    pub fn run(&self, testin: &str, progout: &str) -> Result<(), String> {
        build_and_run("../conf/run/",self.lang, &[], &[testin, progout, self.prog])
    }
} 

use command::build_and_run;

#[allow(dead_code)] 
pub struct Comparator<'a> {
    lang: &'a str,
} 

#[allow(dead_code)] 
impl<'a> Comparator<'a> {
    pub fn new(lang: &'a str) -> Comparator<'a> {
        Comparator { lang: lang }
    }

    pub fn compare(&self, output: &str, expect: &str) -> Result<(), String> {
        build_and_run("../conf/compare/", self.lang, &[], &[output, expect])
    }
}

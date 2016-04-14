extern crate libc;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

mod build;
mod run;
mod compare;
mod command;

use build::Builder;
use run::Runner;
//use std::ffi::CString;
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::io::Write;
use std::fs::File;

#[allow(unused_imports)]
use compare::Comparator;

#[derive(RustcDecodable)]
struct CodePackage {
    lang: String, 
    code: String,
    input: String
}

#[allow(unused_variables)]
fn run_code(code: &str, language: &str, input: &str) -> Result<(), String> {
/*
    unsafe {
        let workdir = CString::new("workspace").unwrap().as_ptr();
        libc::mkdir(workdir, 0o755); 
        if libc::chdir(workdir) != 0 {
            return Err("change dir to 'workspace' failed".to_string());
        }
    }
*/    
    //构建可执行文件
    let builder = Builder::new(code, language);
    try!(builder.build("a.out"));

    
    //运行目标程序, 生成输出
    let runner = Runner::new("a.out", language);
    if let Ok(mut f) = File::create("testin") {
        if let Err(e) = f.write_all(input.as_bytes()) {
            return Err(e.to_string());
        }
    } else {
        return Err("can't create testin".to_string());
    }
    try!(runner.run("testin", "progout"));

    //和期望输出进行比较
//    let comparator = Comparator::new(language);

    Ok(()) 
}

fn main() {
    let mut router = Router::new();
    router.post("/", |request: &mut Request| -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        match json::decode::<CodePackage>(&payload) {
            Ok(code_package) => {
                if let Err(s) = run_code(&code_package.code, &code_package.lang, &code_package.input) {
                    Ok(Response::with((status::Ok, s)))
                } else {
                    let mut output = String::new();
                    let mut outfile = File::open("progout").unwrap();
                    outfile.read_to_string(&mut output).unwrap();
                    Ok(Response::with((status::Ok, output)))
                }
            },
            Err(e) => Ok(Response::with((status::BadRequest, e.to_string())))
        }            
    });

    Iron::new(router).http("localhost:8888").unwrap();
}

mod build;
mod run;
mod compare;

use std::process::*;
use build::Builder;
use run::Runer;
use compare::Comparator;

fn run_tests(code: &str, language: &str, problem: &str) -> Result<(), String> {
    let builder = Builder::new(code, language);
    try!(builder.build());
    let _ = Runer::new();        
    let _ = Comparator::new(); 
    let _ = code; 
    let _ = language; 
    let _ = problem;
    Ok(()) 
}

fn main() {
    if let Err(reason) = run_tests("#include <stdio.h> int main() {}", "c", "789") {
        println!("{}", reason);
    }
}

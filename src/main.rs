#![feature(unboxed_closures)]
extern crate clap;
extern crate readline as rl;
mod shell;
mod eval;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
use clap::{App, Arg};
use shell::Shell;
use eval::evaluate;

fn main() {
    let mut interactive = false;
    let matches = App::new("Calc")
        .version(VERSION)
        .arg(Arg::with_name("interactive")
             .short("i")
             .long("interactive")
             .multiple(true)
             .takes_value(false)
             .help("Run an interactive shell"))
        .arg(Arg::with_name("EXPR")
             .help("Expression to evaluate (e.g. 1+1)")
             .index(1))
        .get_matches();

    if matches.occurrences_of("interactive") > 0 {
        interactive = true;
    }

    if !interactive {
        let mut s: String = String::new();
        match matches.value_of("EXPR") {
            Some(expr) => s.push_str(expr),
            None => return
        }
        match evaluate(&s) {
            Ok(f) => println!("{}", f),
            Err(e) => println!("{:?}", e)
        }
        return
    }

    Shell::new("expr> ")
        .frontload(matches.value_of("EXPR"))
        .function(".help", |expr: &str| -> bool {
            if expr.len() == 0 {
                println!("Supported mathematical operators: +-*/^()");
                println!("Supported functions: sin, cos, tan, asin, acos, atan, floor, ceil");
                println!("Meta-functions: .quit, .help");
                return false;
            }
            match expr {
                "help" => println!("prints the help message"),
                "quit" => println!("exits the shell"),
                _ => println!("...what?")
            }
            false
        })
        .function(".quit", |_| -> bool {true})
        .run(|expr: &String| {
            match evaluate(expr) {
                Ok(f) => println!("{}", f),
                Err(e) => println!("{:?}", e),
            }
            false
        });
}

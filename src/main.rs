#![feature(unboxed_closures)]
extern crate clap;
extern crate readline as rl;

const VERSION: &'static str = env!("CARGO_PKG_VERSION"); //'
use clap::{App, Arg};
use std::io::Write;
use std::io;
mod shell;
mod eval;
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

    if interactive {
        Shell::new("expr> ")
            .frontload(matches.value_of("EXPR"))
            .function(".help", |expr: &str, out: &mut io::Stdout| -> bool {
                if expr.len() == 0 {
                    write!(out, "Supported mathematical operators: +-*/^()\n").unwrap();
                    write!(out, "Supported functions: sin, cos, tan, asin, acos, atan, floor, ceil\n").unwrap();
                    write!(out, "Meta-functions: .quit, .help\n").unwrap();
                    return false;
                }
                match expr {
                    "help" => {
                        write!(out, "prints the help message\n").unwrap();
                    },
                    "quit" => {
                        write!(out, "exits the shell\n").unwrap();
                    },
                    _ => write!(out, "...what?\n").unwrap()
                }
                false
            })
            .function(".quit", |_,_ | -> bool {true})
            .run(|expr: &String, out: &mut io::Stdout| {
                match evaluate(expr) {
                    Ok(f) => write!(out, "{}\n", f).unwrap(),
                    Err(e) => write!(out, "{:?}\n", e).unwrap(),
                }
                false
            });
    }
    else
    {
        let mut s: String = String::new();
        match matches.value_of("EXPR") {
            Some(expr) => s.push_str(expr),
            None => return
        }
        match evaluate(&s) {
            Ok(f) => println!("{}", f),
            Err(e) => println!("{:?}", e)
        }
    }
}

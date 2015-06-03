extern crate readline as rl;
use std::ffi::CString;
use std::io;
use std::io::Write;
use std::collections::hash_map::HashMap;

pub struct Shell<'a> {
    prompt: CString,
    stdout: io::Stdout,
    frontload: Option<&'a str>,
    functions: HashMap<&'a str, Box<FnMut(&str, &mut io::Stdout) -> bool + 'a>>
}

impl<'a> Shell<'a> {
    pub fn new(prompt: &str) -> Shell {
        Shell{prompt: CString::new(prompt).unwrap(), stdout: io::stdout(), frontload: None, functions: HashMap::new()}
    }

    pub fn function<F>(mut self, name: &'a str, func: F) -> Shell<'a>
        where F: FnMut(&str, &mut io::Stdout) -> bool + 'a{
        self.functions.insert(name, Box::new(func));
        self
    }

    fn is_meta(c: &String) -> bool {
        let mut itr = c.chars().peekable();
        let a = itr.next().unwrap();
        let b = itr.peek().unwrap();

        if a == '.' && !b.is_numeric() {
            return true
        }
        false
    }

    fn call_func(&mut self, name: &str, command: &str) -> bool {
        (*(self.functions.get_mut(name).unwrap()))(&command.to_string(), &mut self.stdout)
    }

    pub fn run<F>(&mut self, mut eval: F)
        where F: FnMut(&String, &mut io::Stdout) -> bool {
        let mut s = String::new();
        if !self.frontload.is_none() {
            write!(self.stdout, "{}{}\n", String::from_utf8(self.prompt.to_bytes().to_vec()).unwrap(), &self.frontload.unwrap()).unwrap();
            s.push_str(&self.frontload.unwrap());
            eval(&s, &mut self.stdout);
            rl::add_history(&CString::new(self.frontload.unwrap()).unwrap());
        }
        while let Ok(s) = rl::readline(&self.prompt) {
            let mut rs = String::from_utf8(s.to_bytes().to_vec()).unwrap();
            if rs.len() == 0 {
                continue;
            }

            if Shell::is_meta(&rs) { 
                rs.push(' ');
                let parts = rs.split(" ").collect::<Vec<&str>>();
                if self.functions.contains_key(parts[0]) && Shell::call_func(self, parts[0], parts[1]) {
                    break;
                }
                continue;
            }
            let shrinked = rs.chars().filter(|c: &char| *c != ' ').collect::<String>();
            if eval(&shrinked, &mut self.stdout) == true {
                break;
            }
            rl::add_history(&s);
        }
    }
    pub fn frontload(mut self, expr: Option<&'a str>) -> Shell<'a> {
        self.frontload = expr;
        self
    }
}

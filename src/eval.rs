use std::ops::Range;
use std::string::String;
// Result/Error stuff

#[derive(Debug)]
pub struct EvalError<'a> {
    kind: EvalErrorType,
    message: &'a str
}

#[derive(Debug)]
pub enum EvalErrorType {
    BracketMismatch(usize),
    UnknownFunction(usize),
    NotANumber(usize)
}

enum Function {
    Sin(f64),
    Cos(f64),
    Tan(f64),
    ASin(f64),
    ACos(f64),
    ATan(f64),
    Floor(f64),
    Ceiling(f64),
}

const STR_FUNCS:[&'static str; 8] = ["sin","cos","tan","asin",
                             "acos","atan","floor","ceil"];

#[derive(PartialEq,PartialOrd,Debug)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
    Function,
    Expression,
    Value,
}

fn solve_function(f: Function) -> f64 {
    match f {
        Function::Sin(x) => x.sin(),
        Function::Cos(x) => x.cos(),
        Function::Tan(x) => x.tan(),
        Function::ASin(x) => x.asin(),
        Function::ACos(x) => x.acos(),
        Function::ATan(x) => x.atan(),
        Function::Floor(x) => x.floor(),
        Function::Ceiling(x) => x.ceil(),
    }
}

fn solve_partial(op: Operator, lhs:f64, rhs:f64) -> f64 {
    match op {
        Operator::Addition => lhs + rhs,
        Operator::Subtraction => lhs - rhs,
        Operator::Multiplication => lhs * rhs,
        Operator::Division => lhs / rhs,
        Operator::Exponent => lhs.powf(rhs),
        Operator::Expression => rhs,
        Operator::Value => unreachable!(),
        Operator::Function => unreachable!()
    }
}

fn is_number(c: Option<&(usize, char)>) -> bool {
    match c {
        Some(&(_, c)) => c.is_numeric(),
        None => false
    }
}

fn is_rparen(c: Option<&(usize, char)>) -> bool {
    match c {
        Some(&(_, c)) => (c == ')'),
        None => false
    }
}

fn find_operator(expr: &str, r: Range<usize>) -> Result<(usize, Operator), EvalError> {
    let mut paren = 0;
    let mut priority = Operator::Value;
    let mut pos = 0;
    let mut itr = expr.chars()
        .rev()
        .skip(expr.len()-r.end)
        .take(r.end-r.start)
        .enumerate()
        .peekable();
    

    loop {
        let curr = itr.next();
        let next = itr.peek();
        match curr {
            Some((_, ')')) =>
                paren = paren + 1,
            Some((i, '(')) => {
                paren = paren - 1;
                if paren == 0 && priority > Operator::Expression {
                    priority = Operator::Expression;
                    pos = i;
                }
            },
            Some((i, '+')) => {
                if paren != 0 { continue; }
                if priority > Operator::Addition {
                    priority = Operator::Addition;
                    pos = i;
                }
            },
            Some((i, '-')) => {
                if paren != 0 { continue; }
                if priority > Operator::Subtraction {
                    if is_number(next) || is_rparen(next) {
                        priority = Operator::Subtraction;
                        pos = i;
                    } else { //negative number
                        pos = i;
                    }
                }
            },
            Some((i, '/')) => {
                if paren != 0 { continue; }
                if priority > Operator::Division {
                    priority = Operator::Division;
                    pos = i;
                }
            },
            Some((i, '*')) => {
                if paren != 0 { continue; }
                if priority > Operator::Multiplication {
                    priority = Operator::Multiplication;
                    pos = i;
                }
            },
            Some((i, '^')) => {
                if paren != 0 { continue; }
                if priority > Operator::Exponent {
                    priority = Operator::Exponent;
                    pos = i;
                }
            },
            Some((i, '.')) => {
                if paren != 0 { continue; }
                if priority == Operator::Value {
                    pos = i;
                }
            },
            Some((i, c)) => {
                if priority == Operator::Value && (c.is_numeric() || c == '.') {
                    pos = i;
                    continue;
                }

                if c.is_numeric() || c == '.' || paren != 0 {
                    continue;
                }
                if priority >= Operator::Function {
                    let tmp = (0, '\x00');
                    let n = next.unwrap_or(&tmp);
                    if c.is_alphabetic() && n.1.is_alphabetic() {
                        priority = Operator::Function;
                        pos = i;
                    }
                }
            },
            None => {
                break;
            }
        }
        if priority == Operator::Addition || priority == Operator::Subtraction {
            break;
        }
    }

    if paren != 0 {
        return Err(EvalError{
            kind: EvalErrorType::BracketMismatch(pos),
            message: "I added them up, it's wonky!"
        })
    }

    Ok((r.end-pos-1, priority))
}

fn str_func_to_real(f: &str, x: f64) -> Function {
    match f {
        "sin" => Function::Sin(x),
        "cos" => Function::Cos(x),
        "tan" => Function::Tan(x),
        "asin" => Function::ASin(x),
        "acos" => Function::ACos(x),
        "atan" => Function::ATan(x),
        "floor" => Function::Floor(x),
        "ceil" => Function::Ceiling(x),
        _ => unreachable!()
    }
}

fn parse_function(expr: &String, r: Range<usize>) -> Result<f64, EvalError> {
    let str_name = expr.chars().skip(r.start-1).collect::<String>();
    for f in STR_FUNCS.iter() {
        if str_name.starts_with(f) {
            match parse_expr(expr, Range{start: r.start+f.len(), end: r.end-1}) {
                Ok(x) => return Ok(solve_function(str_func_to_real(f, x))),
                Err(e) => return Err(e)
            }
        }
    }
    Err(EvalError{
        kind: EvalErrorType::UnknownFunction(r.start),
        message: expr})
}

fn parse_number(expr: &String, range: Range<usize>) -> Result<f64, EvalError> {
    let s:String = String::from_utf8(
        expr.chars()
        .skip(range.start)
        .map(|c: char| c as u8)
        .take(range.end-range.start).collect::<Vec<u8>>()).unwrap();
    match s.parse::<f64>() {
        Ok(f) => Ok(f),
        Err(_) => Err(EvalError{
            kind: EvalErrorType::NotANumber(range.start),
            message: "ParseFloatError"
        })
    }
}

fn parse_expr(expr: &String, r: Range<usize>) -> Result<f64, EvalError> {
    let mut lhs:f64 = 0.0;
    let mut rhs:f64 = 0.0;
    let mut pos:usize;
    let mut op;

    match find_operator(expr, Range{..r}) {
        Ok((p, t)) => {
            pos = p;
            op = t;
        },
        Err(e) => return Err(e),
    }

    if op < Operator::Function {
        match parse_expr(expr, Range{end:pos, .. r}) {
            Ok(f) => lhs = f,
            Err(e) => return Err(e)
        }
    }
    
    if op < Operator::Function {
        match parse_expr(expr, Range{start:pos+1, ..r}) {
            Ok(f) => rhs = f,
            Err(e) => return Err(e)
        }
    }

    match op {
        Operator::Addition => Ok(solve_partial(op, lhs, rhs)),
        Operator::Subtraction => Ok(solve_partial(op, lhs, rhs)),
        Operator::Division => Ok(solve_partial(op, lhs, rhs)),
        Operator::Multiplication => Ok(solve_partial(op, lhs, rhs)),
        Operator::Exponent => Ok(solve_partial(op, lhs, rhs)),
        Operator::Expression => parse_expr(expr, Range{start:pos+1, end: r.end-1}),
        Operator::Value => parse_number(expr, Range{start:pos, ..r}),
        Operator::Function => parse_function(expr, Range{start:pos, ..r}),
    }
}

pub fn evaluate(expr: &String) -> Result<f64, EvalError> {
    parse_expr(expr, Range{start: 0, end: expr.len()})
}

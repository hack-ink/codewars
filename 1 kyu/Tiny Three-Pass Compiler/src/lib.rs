#[derive(Clone)]
enum Ast { BinOp(String, Box<Ast>, Box<Ast>), UnOp(String, i32) }

enum Kind { ArgOrder(i32), Num(i32), Op(char) }

use std::collections::HashMap;
use Ast::{BinOp, UnOp};
use Kind::{ArgOrder, Num, Op};

impl Ast {
    fn is_constant(&self) -> bool { if let &UnOp(ref string, _) = self { if string.as_str() == "imm" { return true; } else { return false; } } else { return false; } }

    fn unbox(value: Box<Ast>) -> i32 { if let UnOp(_, value) = *value { return value; } else { unreachable!() } }

    fn calc(a: i32, b: i32, op: char) -> i32 {
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => unreachable!()
        }
    }

    fn simplify(&self) -> Ast {
        if let &BinOp(ref op, ref a, ref b) = self {
            let mut a = a.clone();
            let mut b = b.clone();
            if !a.is_constant() { a = Box::new(a.simplify()); }
            if !b.is_constant() { b = Box::new(b.simplify()); }
            if a.is_constant() && b.is_constant() {
                let a = Ast::unbox(a);
                let b = Ast::unbox(b);
                return UnOp("imm".to_string(), Ast::calc(a, b, op.clone().chars().next().unwrap()));
            } else { return BinOp(op.clone(), a, b); }
        } else { self.clone() }
    }

    fn transform(op: &str) -> String {
        match op {
            "+" => "AD".to_string(),
            "-" => "SU".to_string(),
            "*" => "MU".to_string(),
            "/" => "DI".to_string(),
            _ => unreachable!()
        }
    }

    fn as_string_ary(&mut self) -> Vec<String> {
        let mut result = vec![];
        match self {
            &mut UnOp(ref string, value) =>
                {
                    match string.as_str() {
                        "imm" => result.push(format!("IM {}", value)),
                        "arg" => result.push(format!("AR {}", value)),
                        _ => unreachable!()
                    }
                }
            &mut BinOp(ref op, ref a, ref b) => {
                result.append(&mut a.clone().as_string_ary());
                result.push("PU".to_string());
                result.append(&mut b.clone().as_string_ary());
                result.push("SW".to_string());
                result.push("PO".to_string());
                result.push(Ast::transform(op));
            }
        }
        result
    }
}

struct Compiler { expression: Vec<Kind> }

impl Compiler {
    fn new() -> Compiler { Compiler { expression: Vec::new() } }

    fn tokenize<'a>(&mut self, program: &'a str) -> Vec<String> {
        let mut tokens = vec![];
        let mut args: HashMap<String, i32> = HashMap::new();
        let mut arg_order = 0;
        let mut read_expression = false;
        let mut iter = program.chars().peekable();
        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a' ... 'z' | 'A' ... 'Z' =>
                        {
                            let mut tmp = String::new();
                            while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() { tmp.push(iter.next().unwrap()); }
                            if read_expression {
                                let arg = *args.get(&tmp).unwrap();
                                self.expression.push(ArgOrder(arg));
                            } else {
                                args.insert(tmp.clone(), arg_order);
                                arg_order += 1;
                            }
                            tokens.push(tmp);
                        }
                    '0' ... '9' =>
                        {
                            let mut tmp = String::new();
                            while iter.peek().is_some() && iter.peek().unwrap().is_numeric() { tmp.push(iter.next().unwrap()); }
                            self.expression.push(Num(tmp.parse::<i32>().unwrap()));
                            tokens.push(tmp);
                        }
                    ' ' => { iter.next(); }
                    _ => {
                        let op = iter.next().unwrap();
                        if read_expression { self.expression.push(Op(op)); }
                        if c == ']' { read_expression = true; }
                        tokens.push(op.to_string());
                    }
                },
                None => break
            }
        }
        self.expression.insert(0, Op('('));
        self.expression.push(Op(')'));
        tokens
    }

    fn priority(op: char) -> usize {
        match op {
            '+' => 1,
            '-' => 1,
            '*' => 2,
            '/' => 2,
            _ => unreachable!()
        }
    }

    fn pass1(&mut self, program: &str) -> Ast {
        let _tokens = self.tokenize(program);    //    Useless var, fuck the stupid 'assert_eq' test!! Actually, in tokenize we can just return Vec<Kind>.
        let mut op_stack: Vec<char> = vec![];
        let mut expression: Vec<Ast> = vec![];
        while let Some(pop) = self.expression.pop() {
            match pop {
                Op(op) =>
                    match op {
                        '(' =>
                            while let Some(op) = op_stack.pop() {
                                if op == ')' { break; } else {
                                    let a = expression.pop().unwrap();
                                    let b = expression.pop().unwrap();
                                    expression.push(BinOp(op.to_string(), Box::new(a), Box::new(b)));
                                }
                            },
                        ')' => op_stack.push(op),
                        _ => loop {
                            if op_stack.is_empty() || *op_stack.last().unwrap() == ')' || Compiler::priority(op) >= Compiler::priority(*op_stack.last().unwrap()) {
                                op_stack.push(op);
                                break;
                            } else {
                                let a = expression.pop().unwrap();
                                let b = expression.pop().unwrap();
                                expression.push(BinOp(op_stack.pop().unwrap().to_string(), Box::new(a), Box::new(b)));
                            }
                        }
                    },
                ArgOrder(arg_order) => expression.push(UnOp("arg".to_string(), arg_order)),
                Num(num) => expression.push(UnOp("imm".to_string(), num))
            }
        }
        expression.pop().unwrap()
    }

    fn pass2(&mut self, ast: &Ast) -> Ast { ast.simplify() }

    fn pass3(&mut self, ast: &Ast) -> Vec<String> { ast.clone().as_string_ary() }

    fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }
}

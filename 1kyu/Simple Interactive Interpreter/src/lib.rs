use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum Kind { Num(f32), Op(char), Parameter(String) }

struct Interpreter {
    rpn_expression: Vec<Kind>,
    var_declaration: Vec<String>,
    var_definition: HashMap<String, f32>,
    fn_declaration: Vec<String>,
    fn_parameters: Vec<String>,
    fn_definition: HashMap<String, (Vec<String>, Vec<Kind>)>
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            rpn_expression: vec![],
            var_declaration: vec![],
            var_definition: HashMap::new(),
            fn_declaration: vec![],
            fn_parameters: vec![],
            fn_definition: HashMap::new()
        }
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        let result = self.analyze(input);
        self.rpn_expression.clear();
        self.var_declaration.clear();
        self.fn_declaration.clear();
        self.fn_parameters.clear();
        result
    }

    fn priority(op: char) -> usize {
        match op {
            '#' => 0,
            '=' => 1,
            '+' => 2,
            '-' => 2,
            '*' => 3,
            '/' => 3,
            '%' => 3,
            _ => unreachable!()
        }
    }

    fn calc_fn(&mut self, chained_fn: &mut Vec<(String, usize)>, calc_fn: &mut bool) {
        let fn_name = chained_fn.last().unwrap().clone();
        if fn_name.1 == self.fn_definition.get(&fn_name.0).unwrap().0.len() {
            let num = self.execute_expression(true, &fn_name.0).unwrap().unwrap();
            self.rpn_expression.push(Kind::Num(num));
            chained_fn.pop();
            if chained_fn.is_empty() { *calc_fn = false; } else { self.calc_fn(chained_fn, calc_fn); }
        }
    }

    fn analyze(&mut self, input: &str) -> Result<Option<f32>, String> {
        let input: Vec<char> = input.chars().collect();
        let len = input.len();
        let mut operator_stack = vec!['#'];
        let mut chained_fn: Vec<(String, usize)> = vec![];
        let mut index: usize = 0;
        let mut declare_fn = false;
        let mut define_fn = false;
        let mut calc_fn = false;
        let mut expression_start = 0;
        let mut char;
        while index < len {
            char = input[index];
            match char {
                char if char.is_numeric() =>
                    {
                        let mut string = char.to_string();
                        for &c in input[index + 1..].iter() {
                            if c.is_numeric() || c == '.' {
                                index += 1;
                                string.push(c);
                            } else { break; }
                        }
                        if calc_fn {
                            let index = chained_fn.len() - 1;
                            chained_fn[index].1 += 1;
                        }
                        self.rpn_expression.push(Kind::Num(string.parse::<f32>().unwrap()));
                    }
                '(' =>
                    {
                        expression_start = index;
                        operator_stack.push(char)
                    }
                ')' => while let Some(op) = operator_stack.pop() { if op != '(' { self.rpn_expression.push(Kind::Op(op)); } else { break; } },
                '*' | '/' | '%' | '+' | '-' | '=' =>
                    {
                        if char == '=' && declare_fn {
                            index += 2;
                            define_fn = true;
                            continue;
                        }
                        if let Some(&op) = operator_stack.last() {
                            if op == '(' {
                                operator_stack.push(char);
                            } else if Interpreter::priority(char) > Interpreter::priority(op) {
                                operator_stack.push(char);
                            } else if Interpreter::priority(char) <= Interpreter::priority(op) {
                                if char != '=' { self.rpn_expression.push(Kind::Op(operator_stack.pop().unwrap())); }
                                operator_stack.push(char);
                            }
                        }
                    }
                ' ' => (),
                _ => {
                    let mut string = char.to_string();
                    for &c in input[index + 1..].iter() {
                        if c.is_alphabetic() {
                            index += 1;
                            string.push(c);
                        } else { break; }
                    }
                    let declare_var = len > 1 && index < len - 2 && input[index + 2] == '=';
                    if !declare_fn && !declare_var && self.fn_definition.contains_key(&string) {
                        if !chained_fn.is_empty() {
                            let index = chained_fn.len() - 1;
                            chained_fn[index].1 += 1;
                        }
                        chained_fn.push((string, 0));
                        calc_fn = true;
                    } else {
                        if define_fn {
                            if self.fn_parameters.contains(&string) { self.rpn_expression.push(Kind::Parameter(string)); } else { return Err(String::from("Invalid input.")); }
                        } else {
                            if string == String::from("fn") {
                                if index - 1 > expression_start { return Err(String::from("Invalid input.")); }
                                declare_fn = true;
                            } else {
                                if !declare_fn {
                                    if declare_var {
                                        if self.fn_definition.contains_key(&string) { return Err(String::from("Invalid input.")); }
                                        self.var_declaration.push(string);
                                    } else {
                                        if let Some(&value) = self.var_definition.get(&string) {
                                            self.rpn_expression.push(Kind::Num(value));
                                        } else {
                                            if self.fn_definition.contains_key(&string) { return Err(String::from("Invalid input.")); }
                                            self.var_declaration.push(string);
                                        }
                                    }
                                } else {
                                    if self.fn_declaration.is_empty() {
                                        if self.var_definition.contains_key(&string) { return Err(String::from("Invalid input.")); }
                                        self.fn_declaration.push(string);
                                    } else {
                                        if self.fn_parameters.contains(&string) { return Err(String::from("Invalid input.")); }
                                        self.fn_parameters.push(string);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if calc_fn { self.calc_fn(&mut chained_fn, &mut calc_fn); }
            if index == len - 1 && calc_fn { return Err(String::from("Invalid input.")); }
            index += 1;
        }
        while operator_stack.len() > 1 { self.rpn_expression.push(Kind::Op(operator_stack.pop().unwrap())); }
        self.rpn_expression.reverse();
        if define_fn {
            self.fn_definition.insert(self.fn_declaration.pop().unwrap(), (self.fn_parameters.clone(), self.rpn_expression.clone()));
            return Ok(None);
        }
        self.execute_expression(false, &String::new())
    }

    fn execute_expression(&mut self, calc_fn: bool, fn_name: &String) -> Result<Option<f32>, String> {
        let values = self.fn_definition.get(fn_name);
        let mut parameters_value = vec![];
        let mut stack = vec![];
        let mut rpn_expression;
        let mut parameters: HashMap<String, Kind> = HashMap::new();

        if calc_fn {
            for _ in 0..values.unwrap().0.len() { parameters_value.push(self.rpn_expression.pop().unwrap()); }
            parameters = values.unwrap().0.clone().into_iter().zip(parameters_value.into_iter()).collect();
            rpn_expression = values.unwrap().1.clone();
        } else {
            rpn_expression = self.rpn_expression.clone();
        }
        while let Some(element) = rpn_expression.pop() {
            match element {
                Kind::Num(num) => stack.push(num),
                Kind::Op(op) =>
                    {
                        if op != '=' {
                            let b = stack.pop().unwrap();
                            let a = stack.pop().unwrap();
                            match op {
                                '*' => stack.push(a * b),
                                '/' => stack.push(a / b),
                                '%' => stack.push(a % b),
                                '+' => stack.push(a + b),
                                '-' => stack.push(a - b),
                                _ => unreachable!()
                            }
                        } else {
                            if let Some(&value) = stack.last() {
                                if let Some(var) = self.var_declaration.pop() {
                                    self.var_definition.insert(var, value);
                                } else { return Err(String::from("Invalid input.")); }
                            } else { return Err(String::from("Invalid input.")); }
                        }
                    }
                Kind::Parameter(candidate) => { if let &Kind::Num(num) = parameters.get(&candidate).unwrap() { stack.push(num); } }
            }
        }
        if !calc_fn { self.rpn_expression.clear(); }
        if !self.var_declaration.is_empty() { return Err(String::from("Invalid input.")); }
        if stack.len() > 1 { return Err(String::from("Invalid input.")); }
        Ok(stack.pop())
    }
}
struct Data2D {
    iterations: usize,
    columns: usize,
    rows: usize,
    grid: Vec<Vec<usize>>
}

impl Data2D {
    fn new(iterations: usize, width: usize, height: usize) -> Data2D {
        let mut grid = Vec::new();
        let mut row = Vec::new();
        row.resize(width, 0);
        grid.resize(height, row);
        Data2D {
            iterations,
            columns: width - 1,
            rows: height - 1,
            grid
        }
    }

    fn grid_to_string(&self) -> String {
        let mut result = Vec::new();
        for row in &self.grid { result.push(row.iter().map(|x| x.to_string()).collect::<String>()); }
        return result.join("\r\n");
    }

    fn paint(&mut self, code: Vec<char>, mut x: usize, mut y: usize) -> String {
        if code.is_empty() || self.iterations == 0 { return self.grid_to_string(); }
        let mut times: usize = 0;
        let mut index: usize = 0;
        let mut pass = false;
        let mut ignore_bracket = false;
        let mut jump = false;
        let mut bracket_index: Vec<usize> = Vec::new();
        loop {
            if times == self.iterations || index == code.len() { return self.grid_to_string(); }
            let char = code[index];
            match char {
                char if pass && char != ']' =>
                    {
                        index += 1;
                        continue;
                    }
                'e' => if x == self.columns { x = 0; } else { x += 1; },
                's' => if y == self.rows { y = 0; } else { y += 1; },
                'w' => if x == 0 { x = self.columns; } else { x -= 1; },
                'n' => if y == 0 { y = self.rows; } else { y -= 1; },
                '*' => if self.grid[y][x] == 0 { self.grid[y][x] = 1; } else { self.grid[y][x] = 0; },
                '[' =>
                    if self.grid[y][x] == 0 {
                        pass = true;
                        if jump {
                            jump = false;
                            index += 1;
                            continue;
                        }
                    } else {
                        jump = false;
                        bracket_index.push(index);
                        if ignore_bracket {
                            if code[index + 1] == '[' { ignore_bracket = false; }
                            index += 1;
                            continue;
                        }
                    },
                ']' =>
                    if pass {
                        pass = false;
                        ignore_bracket = false;
                        index += 1;
                        continue;
                    } else {
                        index = bracket_index.pop().unwrap();
                        jump = true;
                        index -= 1;
                        ignore_bracket = true;
                    },
                _ => continue
            }
            index += 1;
            times += 1;
        }
    }
}

fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let mut data = Data2D::new(iterations, width, height);
    data.paint(code.chars().filter(|c| match *c {
        'n' | 's' | 'w' | 'e' | '*' | '[' | ']' => true,
        _ => false
    }).collect::<Vec<char>>(), 0, 0)
}

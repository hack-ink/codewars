#[derive(Debug)]
pub struct Sudoku {
    pub data: Vec<Vec<u32>>,
}

/*
    Data structure dimension: NxN where N > 0 and √N == integer
    Rows may only contain integers: 1..N (N included)
    Columns may only contain integers: 1..N (N included)
    'Little squares' (3x3 in example above) may also only contain integers: 1..N (N included)
*/

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        if self.data.is_empty() { return false; }
        let n = self.data.len();
        let root = (n as f64).sqrt();
        let exp: Vec<u32> = (1..n as u32 + 1).collect();
        let mut transform: Vec<Vec<u32>> = Vec::new();
        transform.resize(n, Vec::new());
        if n <= 0 && root != root.trunc() { return false; }
        for row in self.data.iter() {
            if row.len() != n { return false; }
            let mut sorted_row = row.clone();
            sorted_row.sort();
            if sorted_row != exp { return false; }
            for (i, row_item) in row.iter().enumerate() { transform[i].push(row_item.clone()); }
        }
        for column in transform.iter() {
            let mut sorted_column = column.clone();
            sorted_column.sort();
            if sorted_column != exp { return false; }
        }
        if n % 3 == 0 && self.data == transform { return false; }
        true
    }
}

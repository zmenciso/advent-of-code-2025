use std::error::Error;

const OPS: [char; 2] = ['+', '*'];

#[derive(Debug)]
pub struct Problem {
    pub nums: Vec<Vec<usize>>,
    pub ops: Vec<char>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            nums: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.chars().any(|c| OPS.contains(&c)) {
            let v = line.split_whitespace().flat_map(|s| s.chars()).collect();

            self.ops = v;
        } else {
            let v = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            self.nums.push(v);
        }

        Ok(())
    }

    pub fn compute(&self) -> Vec<usize> {
        assert!(self.nums.len() > 1, "not enough operands");
        let mut v: Vec<usize> = Vec::new();

        for col in 0..self.ops.len() {
            let result = match self.ops[col] {
                '+' => (0..self.nums.len())
                    .map(|row| self.nums[row][col])
                    .sum::<usize>(),
                '*' => (0..self.nums.len())
                    .map(|row| self.nums[row][col])
                    .product::<usize>(),
                _ => 0,
            };

            v.push(result);
        }

        v
    }
}

use regex::Regex;
use std::error::Error;

const OPS: [char; 2] = ['+', '*'];

#[derive(Debug)]
pub struct Problem {
    pub nums: Vec<Vec<char>>,
    pub ops: Vec<char>,
    pub width: Vec<usize>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            nums: Vec::new(),
            ops: Vec::new(),
            width: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.chars().any(|c| OPS.contains(&c)) {
            let ops: Vec<char> = line.split_whitespace().flat_map(|s| s.chars()).collect();
            let re = Regex::new(r"\S(\s+)")?;

            let width = re
                .captures_iter(line)
                .map(|caps| caps.get(1).unwrap().as_str().len())
                .collect();

            self.ops = ops;
            self.width = width;
        } else {
            let v: Vec<char> = line.chars().collect();
            self.nums.push(v);
        }

        Ok(())
    }

    pub fn compute(&self) -> Vec<usize> {
        let mut results: Vec<usize> = Vec::with_capacity(self.ops.len());
        let mut i = 0usize;

        for (idx_op, op) in self.ops.iter().enumerate() {
            let n = self.width[idx_op];

            // First, build a vector with all the values in the "column"
            let mut v: Vec<usize> = Vec::with_capacity(self.nums.len());

            for col in (i..i + n).rev() {
                let num: usize = self.nums.iter().fold(0usize, |acc, row| {
                    if row[col] == ' ' {
                        acc
                    } else {
                        let digit = row[col].to_digit(10).unwrap() as usize;
                        acc * 10 + digit
                    }
                });

                v.push(num);
            }

            // Also move over the space between columns
            i += n + 1;

            let result = match op {
                '+' => v.iter().sum::<usize>(),
                '*' => v.iter().product::<usize>(),
                _ => 0,
            };

            println!("{op} {:?} -> {result}", v);

            results.push(result);
        }

        results
    }
}

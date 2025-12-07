use std::error::Error;

const OPS: [char; 2] = ['+', '*'];
const PAD: usize = 10;

#[derive(Debug)]
pub struct Problem {
    pub nums: Vec<Vec<char>>,
    pub ops: Vec<char>,
    pub width: Vec<usize>,
}

fn run_lengths(row: &Vec<char>) -> Vec<usize> {
    let mut lengths = Vec::new();
    let mut curr = 0usize;

    for c in row {
        if c.is_whitespace() {
            if curr > 0 {
                lengths.push(curr);
                curr = 0;
            }
        } else {
            curr += 1;
        }
    }

    if curr > 0 {
        lengths.push(curr);
    }

    lengths
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
            let mut width: Vec<usize> = vec![0usize; ops.len()];

            for row in &self.nums {
                let lengths = run_lengths(row);

                for (i, &len) in lengths.iter().enumerate() {
                    if len > width[i] {
                        width[i] = len;
                    }
                }
            }

            self.ops = ops;
            self.width = width;
        } else {
            let mut v: Vec<char> = line.chars().collect();

            // Pad the end to prevent bounding issues
            // There is a safer, more idiomatic way to do this
            v.extend(std::iter::repeat(' ').take(PAD));

            self.nums.push(v);
        }

        Ok(())
    }

    pub fn compute(&self) -> Vec<usize> {
        let mut results: Vec<usize> = Vec::with_capacity(self.ops.len());
        let mut i = 0usize;

        for (idx_op, op) in self.ops.iter().enumerate() {
            let n = self.width[idx_op];

            // println!("n: {n}, idx_op: {idx_op}, op: {op}, i: {i}");

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

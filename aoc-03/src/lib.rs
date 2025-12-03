const ACTIVE: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LocValue<T> {
    loc: usize,
    value: T,
}

#[derive(Debug)]
pub struct Bank {
    batteries: Vec<usize>,
    n: usize,
}

impl Bank {
    pub fn new(s: &str) -> Bank {
        let digits: Vec<usize> = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect();

        let n = digits.len();
        assert!(n >= ACTIVE, "bank too smol");

        Bank {
            batteries: digits,
            n: n,
        }
    }

    pub fn max_joltage(&self) -> usize {
        let mut active: Vec<LocValue<usize>> = self
            .batteries
            .iter()
            .enumerate()
            .skip(self.n.saturating_sub(ACTIVE))
            .map(|(i, &val)| LocValue { loc: i, value: val })
            .collect();

        let mut start: usize = 0;

        // For each digit in active, slide it as far left as it will go
        for batt in &mut active {
            start = maximize_digit_left(batt, self, start);
        }

        // Compute the max joltage
        active
            .iter()
            .map(|lv| lv.value)
            .fold(0usize, |acc, val| acc * 10 + val)
    }
}

fn maximize_digit_left(batt: &mut LocValue<usize>, bank: &Bank, start: usize) -> usize {
    let inspect = &bank.batteries[start..batt.loc];
    let mut new_loc: Option<usize> = None;

    for (i, &val) in inspect.iter().enumerate() {
        if val > batt.value {
            new_loc = Some(start + i);
            batt.value = val;
        } else if val == batt.value {
            if new_loc.is_none() {
                new_loc = Some(start + i);
            }
        }
    }

    // Return the new stop value, which is just the loc + 1
    batt.loc = match new_loc {
        Some(l) => l,
        None => batt.loc,
    };

    batt.loc + 1
}

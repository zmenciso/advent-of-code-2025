// const ACTIVE: usize = 2;

#[derive(Debug)]
pub struct Bank {
    batteries: Vec<usize>,
}

impl Bank {
    pub fn new(s: &str) -> Bank {
        let digits: Vec<usize> = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect();

        Bank { batteries: digits }
    }

    pub fn max_joltage(self) -> usize {
        let mut batt = self.batteries.iter().peekable();

        let mut head = batt.next().expect("bank missing first battery");
        let mut tail: Option<&usize> = None;

        while let Some(j) = batt.next() {
            // println!("Head: {:?} Tail: {:?}", head, tail);
            // No tail, take whatever is next
            if tail.is_none() {
                tail = Some(j);
            }

            // Not at the end, updating the head is valid
            if batt.peek().is_some() {
                if j > head {
                    head = j;
                    tail = None;
                } else if j > tail.unwrap() {
                    tail = Some(j);
                }
            }
            // At the end, only update the tail
            else {
                if j > tail.unwrap() {
                    tail = Some(j);
                }
            }
        }

        let joltage = head * 10 + tail.unwrap();
        // println!("Done. Head: {:?} Tail: {:?} Joltage: {joltage}", head, tail);
        joltage
    }
}

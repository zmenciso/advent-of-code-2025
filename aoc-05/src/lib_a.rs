use std::error::Error;

pub const DELIM: char = '-';

#[derive(Debug)]
pub struct Range {
    pub min: usize,
    pub max: usize,
}

impl Range {
    pub fn from_str(s: &str) -> Result<Range, Box<dyn Error>> {
        let (start, end) = s.split_once(DELIM).ok_or_else(|| "malformed range")?;
        let (start, end) = (start.parse::<usize>()?, end.parse::<usize>()?);

        Ok(Range {
            min: start,
            max: end,
        })
    }

    pub fn includes(&self, val: &usize) -> bool {
        *val <= self.max && *val >= self.min
    }
}

#[derive(Debug)]
pub struct Ingredients {
    ids: Vec<Range>,
}

impl Ingredients {
    pub fn new() -> Ingredients {
        Ingredients { ids: Vec::new() }
    }

    pub fn fresh(&self, search: &usize) -> bool {
        self.ids.iter().any(|r| r.includes(search))
    }

    pub fn add_range(&mut self, range: &str) -> Result<(), Box<dyn Error>> {
        let range = Range::from_str(range)?;
        self.ids.push(range);

        Ok(())
    }
}

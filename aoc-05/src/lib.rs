use std::error::Error;

pub const DELIM: char = '-';

#[derive(Debug, Copy, Clone)]
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

    pub fn len(&self) -> usize {
        self.max - self.min + 1
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
        let mut new = Range::from_str(range)?;
        let mut dirty = true;

        let mut i = 0usize;

        while dirty {
            dirty = false;

            while i < self.ids.len() {
                let inspect = self.ids[i];

                let (clip_left, clip_right) =
                    (inspect.includes(&new.min), inspect.includes(&new.max));
                let subsume = new.includes(&inspect.min) && new.includes(&inspect.max);

                // New Range is entirely in inspected Range, so we discard it
                if clip_left && clip_right {
                    return Ok(());
                }

                if clip_left {
                    new = Range {
                        min: inspect.min,
                        max: new.max,
                    };
                } else if clip_right {
                    new = Range {
                        min: new.min,
                        max: inspect.max,
                    };
                }

                if clip_left || clip_right || subsume {
                    dirty = true;
                    self.ids.remove(i);
                    break;
                }

                i += 1;
            }
        }

        self.ids.push(new);

        Ok(())
    }

    pub fn total(&self) -> usize {
        let mut count = 0usize;
        self.ids.iter().for_each(|r| count += r.len());

        count
    }
}

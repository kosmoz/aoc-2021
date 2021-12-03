use crate::error::Error;
pub struct Line {
    pub values: [i32; 12],
}

impl Line {
    pub fn parse(s: String) -> Result<Line, Error> {
        let mut values = [0; 12];
        let mut i = 0;
        for c in s.trim().chars().take(12) {
            values[i] = match c {
                '1' => 1,
                '0' => 0,
                _ => return Err(Error::CustomError(format!("unhandled character: '{}'", c))),
            };
            i += 1;
        }
        Ok(Line { values })
    }
}

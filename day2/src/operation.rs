use crate::error::Error;

pub enum Operation {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl Operation {
    pub fn parse(value: String) -> Result<Self, Error> {
        let mut split = value.split(" ");
        let op = split
            .next()
            .ok_or(Error::CustomError("missing operation".to_string()))?;
        let amount = split
            .next()
            .ok_or(Error::CustomError("missing amount".to_string()))?
            .trim()
            .parse::<i64>()?;
        match op {
            "up" => Ok(Operation::Up(amount)),
            "down" => Ok(Operation::Down(amount)),
            "forward" => Ok(Operation::Forward(amount)),
            _ => Err(Error::CustomError("unknown operation".to_string())),
        }
    }
}

use crate::operation::Operation;

pub struct Submarine {
    horizontal: i64,
    depth: i64,
}

impl Submarine {
    pub fn position(self) -> i64 {
        self.horizontal * self.depth
    }

    pub fn new() -> Submarine {
        Submarine {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn apply(self, op: Operation) -> Submarine {
        match op {
            Operation::Up(a) => Submarine {
                depth: self.depth - a,
                horizontal: self.horizontal,
            },
            Operation::Down(a) => Submarine {
                depth: self.depth + a,
                horizontal: self.horizontal,
            },
            Operation::Forward(a) => Submarine {
                depth: self.depth,
                horizontal: self.horizontal + a,
            },
        }
    }
}

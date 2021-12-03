use crate::operation::Operation;

pub struct Submarine {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Submarine {
    pub fn position(self) -> i64 {
        self.horizontal * self.depth
    }

    pub fn new() -> Submarine {
        Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn adjust_aim(self, by: i64) -> Submarine {
        Submarine {
            depth: self.depth,
            horizontal: self.horizontal,
            aim: self.aim + by,
        }
    }

    fn move_forward(self, by: i64) -> Submarine {
        Submarine {
            horizontal: self.horizontal + by,
            depth: self.depth + by * self.aim,
            aim: self.aim,
        }
    }

    pub fn apply(self, op: Operation) -> Submarine {
        match op {
            Operation::Up(a) => self.adjust_aim(-a),
            Operation::Down(a) => self.adjust_aim(a),
            Operation::Forward(a) => self.move_forward(a),
        }
    }
}

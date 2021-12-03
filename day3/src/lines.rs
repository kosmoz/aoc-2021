use crate::line::Line;

pub struct Lines {
    ones: Line,
    total: Line,
}

impl Lines {
    pub fn add(&mut self, other: Line) {
        for i in 0..12 {
            self.ones.values[i] += other.values[i];
            self.total.values[i] += 1
        }
    }

    fn gamma(&self) -> u32 {
        let mut result = 0;
        for i in 0..12 {
            result <<= 1;
            result |= (self.total.values[i] - self.ones.values[i] < self.ones.values[i]) as u32
        }
        result
    }

    pub fn prod(&self) -> u32 {
        let g = self.gamma();
        g * (!g & 0xfffu32)
    }

    pub fn print_debug(&self) {
        print!("ones:");
        for i in 0..12 {
            print!(" {}", self.ones.values[i])
        }
        print!("\ntotal:");
        for i in 0..12 {
            print!(" {}", self.total.values[i])
        }
        println!(
            "\ngamma: {:#012b}, epsilon: {:#012b}\n",
            self.gamma(),
            !self.gamma() & 0xfffu32
        )
    }

    pub fn new() -> Lines {
        Lines {
            ones: Line { values: [0; 12] },
            total: Line { values: [0; 12] },
        }
    }
}

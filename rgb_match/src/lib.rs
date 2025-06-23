#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if self.r == first {
            self.r = second;
        } else if self.r == second {
            self.r = first;
        }

        if self.g == first {
            self.g = second;
        } else if self.g == second {
            self.g = first;
        }

        if self.b == first {
            self.b = second;
        } else if self.b == second {
            self.b = first;
        }

        if self.a == first {
            self.a = second;
        } else if self.a == second {
            self.a = first;
        }

        self
    }
}
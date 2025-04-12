#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if first == self.r || first == self.g || first == self.b || first == self.a {
            if second == self.r || second == self.g || second == self.b || second == self.a {
                // Swap the values
                if first == self.r {
                    self.r = second;
                } else if first == self.g {
                    self.g = second;
                } else if first == self.b {
                    self.b = second;
                } else {
                    self.a = second;
                }
                if second == self.r {
                    self.r = first;
                } else if second == self.g {
                    self.g = first;
                } else if second == self.b {
                    self.b = first;
                } else {
                    self.a = first;
                }
            }
        }
        self
    }
}

// Explanation:
// Struct Color: We have the Color struct with the r, g, b, and a channels, all of type u8.

// Swap Logic:

// We check if both first and second are valid color components (r, g, b, or a).

// If valid, we perform the swap operation between first and second values.

// After the swap, we return the updated Color struct.
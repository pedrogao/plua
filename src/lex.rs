#[derive(Copy, Clone, Debug)]
pub struct Location {
    col: i32,     // 列号
    line: i32,    // 行号
    index: usize, // 字节序好
}

impl Location {
    fn increment(&self, newline: bool) -> Location {
        if newline {
            Location {
                index: self.index + 1,
                col: 0,
                line: self.line + 1,
            }
        } else {
            Location {
                index: self.index + 1,
                col: self.col + 1,
                line: self.line,
            }
        }
    }
}

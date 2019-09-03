// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

struct Reader<'a> {
    lines: Vec<&'a str>,
    row: usize,
    col: usize,
}

impl<'a> Reader<'a> {
    fn new(lines: Vec<&'a str>) -> Self {
        Reader { lines, row: 0, col: 0 }
    }
    fn read_letter(&mut self) -> Option<Result<String, Error>> {
        dbg!(self.row,self.col);
        if &self.lines[self.row].len() <= &self.col {
            if self.lines.len() > self.row + 4 {
                self.row += 4;
                self.col = 0;
                return Some(Ok(",".to_string()))
            } else {
                return None;
            }
        }
        let letter =
            (&self.lines[self.row + 0][self.col..self.col + 3],
             &self.lines[self.row + 1][self.col..self.col + 3],
             &self.lines[self.row + 2][self.col..self.col + 3],
             &self.lines[self.row + 3][self.col..self.col + 3]);

        self.col += 3;

        let res = match letter {
            (" _ ", "| |", "|_|", "   ") => "0".to_string(),
            ("   ", "  |", "  |", "   ") => "1".to_string(),
            (" _ ", " _|", "|_ ", "   ") => "2".to_string(),
            (" _ ", " _|", " _|", "   ") => "3".to_string(),
            ("   ", "|_|", "  |", "   ") => "4".to_string(),
            (" _ ", "|_ ", " _|", "   ") => "5".to_string(),
            (" _ ", "|_ ", "|_|", "   ") => "6".to_string(),
            (" _ ", "  |", "  |", "   ") => "7".to_string(),
            (" _ ", "|_|", "|_|", "   ") => "8".to_string(),
            (" _ ", "|_|", " _|", "   ") => "9".to_string(),
            _ => "?".to_string(),
        };
        Some(Ok(res))
    }
}

impl<'a> Iterator for Reader<'_> {
    type Item = Result<String, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.read_letter()
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    let nrows = lines[0].len();
    if nrows % 3 != 0 {
        return Err(Error::InvalidColumnCount(nrows));
    }

    let mut reader = Reader::new(lines);
    reader.into_iter().collect()
}


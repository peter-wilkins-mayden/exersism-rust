
pub fn get_diamond(ch: char) -> Vec<String> {
    let n = (ch as u8 - b'A') as i8;
    (-n..=n).map(|row| {
        (-n..=n).map(|col| {
            match row.abs() + col.abs() == n {
                true => (col.abs() as u8 + b'A') as char,
                false => ' '
            }
        }).collect()
    }).collect()
}

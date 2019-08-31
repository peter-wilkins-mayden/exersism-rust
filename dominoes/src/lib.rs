pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 1 && input[0].0 != input[0].1 {
        return None;
    }
    Some(input.to_vec())
}

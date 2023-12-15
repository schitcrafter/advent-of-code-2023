pub mod part1;
pub mod part2;

fn hash(input: &str) -> u32 {
    let mut output = 0u32;
    for char in input.chars() {
        output += char as u32;
        output *= 17;
        output = output % 256;
    }
    debug_assert!(output < 256);
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_test() {
        let input = "HASH";
        assert_eq!(52, hash(input));
    }
}

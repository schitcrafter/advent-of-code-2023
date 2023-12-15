use super::*;

pub fn process(input: &str) -> u32 {
    input.split(",")
        .map(|s| s.trim())
        .fold(0, |sum, element| hash(element) + sum)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn example() {
        let test_input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(1320, process(test_input));
    }
}
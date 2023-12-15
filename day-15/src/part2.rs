#![allow(dead_code, unused)]

use std::str::FromStr;

use crate::hash;

pub fn process(input: &str) -> usize {
    let mut boxes = vec![LensBox::new(); 256];
    let input = parse_input(input);
    for operation in input {
        let current_box = &mut boxes[operation.box_hash as usize];

        if let OperationType::Add { focal_length } = operation.operation_type {
            current_box.add_lens(operation.lens_name, focal_length)
        } else {
            current_box.remove_lens(operation.lens_name);
        }
    }

    boxes.iter().enumerate()
        .map(|(box_index, lens_box)| {
            lens_box.lenses.iter().enumerate()
            .map(move |(lens_index, lens)| {
                let focal_length = lens.1 as usize;
                let result = (box_index + 1) * (lens_index + 1) * focal_length;
                result
            })
        })
        .flatten()
        .sum()

}

fn parse_input(input: &str) -> Vec<Operation> {
    input
        .split(",")
        .map(|operation_str| operation_str.parse().unwrap())
        .collect()
}

#[derive(Clone, Debug, Default)]
struct LensBox {
    pub lenses: Vec<(String, u8)>,
}

impl LensBox {
    pub fn remove_lens(&mut self, lens_name: String) {
        self.lenses.retain(|lens| lens.0 != lens_name);
    }

    pub fn add_lens(&mut self, lens_name: String, focal_length: u8) {
        if let Some(mut lens) = self.lenses.iter_mut().find(|lens| lens.0 == lens_name) {
            // if a lens with this name already exists, change it's focal length
            lens.1 = focal_length;
        } else {
            // if no lens exists with this name, add it
            self.lenses.push((lens_name, focal_length));
        }

    }
}

impl LensBox {
    pub const fn new() -> LensBox {
        LensBox { lenses: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]
struct Operation {
    pub box_hash: u32,
    pub lens_name: String,
    pub operation_type: OperationType,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation_type_start = s.find(|char| char == '-' || char == '=').unwrap();
        let box_hash_str = &s[..operation_type_start];

        let operation_type_str = &s[operation_type_start..];
        let operation_type = operation_type_str.parse().unwrap();

        Ok(Operation {
            box_hash: hash(box_hash_str),
            lens_name: box_hash_str.to_string(),
            operation_type,
        })
    }
}

#[derive(Debug, PartialEq)]
enum OperationType {
    Remove,
    Add { focal_length: u8 },
}

impl FromStr for OperationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(OperationType::Remove)
        } else if s.starts_with("=") {
            let focal_length = s[1..].parse().unwrap();
            Ok(OperationType::Add { focal_length })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let test_input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        let result = process(test_input);
        assert_eq!(145, result);
    }

    #[test]
    fn test_operation_parsing() {
        let test_input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        let parsed = parse_input(test_input);
        println!("{parsed:#?}");
    }
}

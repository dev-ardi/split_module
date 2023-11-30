use std::{fs::read_to_string, path::PathBuf, time::Instant};
use clap::Parser;
use serde_json::Value;

#[derive(Debug, Parser)]
struct Cli {
    input: PathBuf,
    split: String,
}

fn main() {
    let t0 = Instant::now();
    let Cli { input, split } = Cli::parse();
    let split = split.split(".").collect::<Vec<_>>();
    assert!(split.len() > 0);

    let input = read_to_string(input).unwrap();
    let input: Value = serde_json::from_str(&input).unwrap();
    assert!(input.is_object());

    let mut curr: &Value = &input;
    for s in split.clone() {
        curr = curr.as_object().expect("not an object").get(s).unwrap();
    }

    let arr = curr.as_array().unwrap();
    let out = arr.into_iter().map(|val| {
        let mut i = input.clone();
        let mut curr: &mut Value = &mut i;
        for s in split.clone() {
            curr = curr.as_object_mut().unwrap().get_mut(s).unwrap();
        }
        *curr = Value::from(vec![val.clone()]);
        i
    });
    for (n, i) in out.enumerate() {
        let name = format!("output{n}.json");
        std::fs::write(name, i.to_string()).unwrap();
    }
    eprintln!("done in {:?}",t0.elapsed())
}

use std::env::args;
use std::path::PathBuf;

use ch32_metapac_gen::*;

fn main() {
    let out_dir = PathBuf::from("build/ch32-metapac");
    let data_dir = PathBuf::from("build/data");

    let args: Vec<String> = args().collect();

    let mut chips = match &args[..] {
        [_, chip] => {
            vec![chip.clone()]
        }
        [_] => std::fs::read_dir(data_dir.join("chips"))
            .unwrap()
            .filter_map(|res| res.unwrap().file_name().to_str().map(|s| s.to_string()))
            .filter(|s| s.ends_with(".json"))
            .map(|s| s.strip_suffix(".json").unwrap().to_string())
            .collect(),
        _ => args[1..].iter().map(|s| s.clone()).collect(),
    };

    chips.sort();

    println!("chips: {:?}", chips);

    let opts = Options {
        out_dir,
        data_dir,
        chips,
    };
    Gen::new(opts).gen();
}

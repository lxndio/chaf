#[macro_use]
extern crate clap;
extern crate glob;

use std::fs::File;
use std::io::{BufReader, Read};

use clap::App;
use glob::glob;
use hashbrown::HashMap;

fn main() {
    let clap_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    let paths: Vec<String> = matches
        .values_of("files")
        .unwrap()
        .map(|x| x.to_string())
        .collect();

    for path in paths {
        for entry in glob(&path).expect("Could not glob") {
            match entry {
                Ok(path) => {
                    let f = File::open(&path).expect("Could not open file");
                    let mut br = BufReader::new(f);
                    let mut data = String::new();

                    br.read_to_string(&mut data).expect("Could not read string");
                    let freqs = count_freqs(&data);
                    let freqs = sort_freqs(freqs);

                    println!("{}:", path.display());
                    for (c, freq) in freqs.iter() {
                        let c = match c {
                            ' ' => String::from("SPACE"),
                            '\n' => String::from("\\n"),
                            '\r' => String::from("\\r"),
                            c => c.to_string(),
                        };
                        println!("\t{}\t{}", c, freq);
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn count_freqs(data: &str) -> HashMap<char, usize> {
    let mut freqs = HashMap::new();

    for c in data.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }

    freqs
}

fn sort_freqs(freqs: HashMap<char, usize>) -> Vec<(char, usize)> {
    let mut freqs: Vec<_> = freqs.iter().map(|(c, freq)| (*c, *freq)).collect();
    freqs.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    freqs
}

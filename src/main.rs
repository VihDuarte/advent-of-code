use crate::{challenge::Challenge, historian_hysteria::HistorianHysteria};

mod challenge;
mod historian_hysteria;

fn main() {
    let challenges = vec![HistorianHysteria::new()];

    for challenge in challenges {
        let input_path = format!("inputs/{}.txt", challenge.name());
        let input = std::fs::read(input_path).expect("sorry about this expect hehe");

        let parsed_input = challenge.parse(&input);
        let output = challenge.solve(&parsed_input);
        println!("{}: {:?}", challenge.name(), output);
    }
}

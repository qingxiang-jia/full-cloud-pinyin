mod fcpinyin;

use std::{
    io::{stdin, stdout, Write}
};

use fcpinyin::FullCloudPinyin;


fn main() {
}

fn interactive_loop() {
    let fcp = FullCloudPinyin::new();
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a corect string.");

        let candidates = fcp.get_candidates(&input, 11);

        println!("{:#?}", candidates);
    }
}

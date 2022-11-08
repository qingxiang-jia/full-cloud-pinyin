use std::io::{stdin, stdout, Write};

use fcp::FullCloudPinyin;


fn main() {
    interactive_loop();
}

fn interactive_loop() {
    let fcp = FullCloudPinyin::new();
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a corect string.");

        let candidates = fcp.query_candidates(&input);

        println!("{:#?}", candidates);
    }
}

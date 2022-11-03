use regex::Regex;
use std::fs;

fn main() {
    let code = fs::read_to_string("input.cc").expect("Failed to load the file.");

    // Set up regex to extract code
    let re_cxx = Regex::new("(namespace rust \\{(?:.|\n)+?\\} // namespace\n\\} // namespace cxxbridge1\n\\} // namespace rust)").expect("re_cxx is invalid.");
    let re_fcp = Regex::new("namespace fcp \\{(?:.|\n)+").expect("re_fcp is invalid.");
    let re_include = Regex::new("(#include (.|\n)+<utility>)").expect("Invalid regex");

    // Extact code
    let cxx = re_cxx
        .captures_iter(&code)
        .nth(0)
        .expect("No match for CXX code.")
        .iter()
        .nth(0)
        .expect("No sub match for CXX code.")
        .expect("Sub match for CXX contains no result.")
        .as_str()
        .to_owned();

    let fcp = re_fcp
        .captures_iter(&code)
        .nth(0)
        .expect("No match for fcp code.")
        .iter()
        .nth(0)
        .expect("No sub match for fcp code.")
        .expect("Sub match for fcp contains no result.")
        .as_str()
        .to_owned();

    let include = re_include
        .captures_iter(&code)
        .nth(0)
        .expect("No match for include code.")
        .iter()
        .nth(0)
        .expect("No sub match for include code.")
        .expect("Sub match for include contains no result.")
        .as_str()
        .to_owned();

    // Define output filenames

    let cxx_filename = "cxx.h";
    let fcp_filename = "ffi.h";

    // Separate include to built-in C++ headers and user headers

    let include_vec: Vec<&str> = include.lines().collect::<Vec<&str>>();
    let include_base: Vec<&&str> = include_vec
        .iter()
        .filter(|directive| !directive.contains("\""))
        .collect();
    let include_user: Vec<&&str> = include_vec
        .iter()
        .filter(|directive| directive.contains("\""))
        .collect();
    
    // Assemble output code
    
    let mut final_cxx = "".to_owned();
    final_cxx.push_str("#pragma once\n");
    for directive in &include_base {
        final_cxx.push_str(directive);
        final_cxx.push_str("\n");
    }
    final_cxx.push_str(&cxx);
    final_cxx.push_str("\n");

    let mut final_fcp = "".to_owned();
    final_fcp.push_str("#pragma once\n");
    for directive in &include_user {
        final_fcp.push_str(directive);
        final_fcp.push_str("\n");
    }
    final_fcp.push_str(&format!("#include\"{}\"\n", cxx_filename));
    for directive in &include_base {
        final_fcp.push_str(directive);
        final_fcp.push_str("\n");
    }
    final_fcp.push_str(&fcp);

    // Write to files
    fs::write(cxx_filename, final_cxx).expect(&format!("Faied to write file {}", cxx_filename));
    fs::write(fcp_filename, final_fcp).expect(&format!("Faied to write file {}", fcp_filename));
}

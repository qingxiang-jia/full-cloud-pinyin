use cmd_lib;
use cmd_lib::run_cmd;
use cmd_lib::run_fun;
use regex::Regex;
use std::env;
use std::fs;
use std::str;

fn main() {
    // Read line from file
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        println!("No argument passed in");
        return;
    }

    let arg2 = &args[2]; // 0 is cargo; 1 is the program itself; 2 is arg

    match arg2.as_str() {
        "gen-cxx" => {
            let code = run_fun!(cxxbridge src/core/ffi.rs).expect("Failed to run cxxbridge.");
            cxx_gen_cc_to_separate_h(code);
        }
        "gen-lib" => {
            let output = run_fun!(cargo rustc --lib --release --crate-type staticlib)
                .expect("Failed to run cargo.");
            println!("{}", output);
        }
        "init" => {
            let pwd = run_fun!(pwd).expect("Failed to run pwd."); // Workaround the cd bug of cmd_lib
            if let Err(err) = run_cmd! {
                rm -rf ../fcitx5/build;
                mkdir ../fcitx5/build;
                cd $pwd/../fcitx5/build;
                pwd;
                cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..;
            } {
                println!("{}", err);
            }
        }
        "build" => {
            if let Err(err) = run_cmd! {
                cd ../fcitx5;
                cmake --build ./build;
            } {
                println!("{}", err);
            }
        }
        "install" => {
            if let Err(err) = run_cmd! {
                cd ../fcitx5;
                sudo cmake --install ./build;
            } {
                println!("{}", err);
            }
        }
        "uninstall" => {
            if let Err(err) = run_cmd! {
                cd ../fcitx5;
                bash -c "sudo xargs -I{} rm {} < ./build/install_manifest.txt"; // Wordaround {} in command
            } {
                println!("{}", err);
            }
        }
        &_ => {
            println!("No such option.");
            return;
        }
    }
}

fn cxx_gen_cc_to_separate_h(code: String) {
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

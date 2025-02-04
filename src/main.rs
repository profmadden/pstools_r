/// PSTools
/// A simple library for generating PostScript graphics.  The raw PostScript
/// can then be converted to PDF using a tool like GhostScript.
use pstools::*;

use argh::FromArgs;
#[derive(FromArgs)]
/// PSTools_r Simplified PostScript generator
struct PSArgs {
    /// input file
    #[argh(option, short = 'i')]
    input: Option<String>,
    /// output file
    #[argh(option, short = 'o')]
    output: Option<String>,
    /// demo mode
    #[argh(switch, short = 'd')]
    demo: bool,
    /// detailed help information
    #[argh(switch, short = 'h')]
    detail: bool,
}
fn main() {
    println!("PSTools Simplified PostScript generation in Rust");
    let arguments: PSArgs = argh::from_env();

    if arguments.detail {
        detailed_help();
        return;
    }

    let mut pst = PSTool::new();
    if arguments.input.is_some() {
        println!("Read an input file");
        pst.parse(&arguments.input.unwrap());
    }
    if arguments.demo {
        pst.demo();
    }
    if pst.len() > 0 {
        if arguments.output.is_some() {
            pst.generate(arguments.output.unwrap());
        } else {
            pst.generate("".to_string());
        }
        return;
    } else {
        println!("Use -h for information.");
    }
}

fn detailed_help() {
    println!("More info and details");
}

use pstools_r::{*};

use argh::FromArgs;
#[derive(FromArgs)]
/// PSTools_r Simplified PostScript generator
struct PSArgs {
    /// input file
    #[argh(option, short='i')]
    input: Option<String>,
    /// output file
    #[argh(option, short='o')]
    output: Option<String>,
    /// demo mode
    #[argh(switch, short='d')]
    demo: bool,
    /// detailed help information
    #[argh(switch, short='h')]
    detail: bool,
}
fn main() {
    println!("PSTools_r Simplified PostScript generation in Rust");
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
        demo(&mut pst);
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

fn demo(pst: &mut PSTool) {
    //let mut pst = PSTool::new();
    pst.add_comment("This text is inserted directly into the PS file".to_string());
    pst.set_fill(true);
    pst.set_color(0.3, 0.4, 0.2, 1.0);
    pst.add_box(5.0, 5.0, 20.0, 30.0);
    pst.set_color(0.8, 0.1, 0.2, 1.0);
    pst.add_box(35.0, 19.0, 7.0, 16.0);
    pst.add_line(33.2, 44.1, 8.7, 5.5);
    
    pst.set_color(0.1, 0.1, 0.8, 1.0);
    pst.set_fill(false);
    pst.add_box(50.0, 50.0, 60.0, 60.0);

    pst.set_color(0.1, 0.1, 0.8, 1.0);
    pst.set_fill(true);
    pst.add_box(70.0, 70.0, 80.0, 80.0);

    pst.set_color(0.0, 0.0, 0.0, 1.0);
    pst.add_text(10.0, 10.0, "1Hello World".to_string());
    
    pst.set_color(0.0, 0.0, 1.0, 1.0);
    pst.set_font(20.0, "Times-Roman".to_string());
    pst.add_text(10.0, 30.0, "2Hello World".to_string());

    pst.set_font(20.0, "Helvetica".to_string());
    pst.set_color(0.0, 1.0, 0.0, 1.0);
    pst.add_text(10.0, 50.0, "3Hello World".to_string());
    
    pst.set_border(10.0);

    pst.add_circle(20.0, 20.0, 8.0);
    pst.set_color(1.0, 0.0, 0.0, 1.0);
    pst.set_fill(false);
    pst.add_circle(22.0, 22.0, 9.0);

    pst.set_color(0.0, 0.0, 0.0, 1.0);
    for i in 1..5 {
        pst.add_curve(5.0, 5.0, 20.0, 30.0 + (i*4) as f32, 50.0, 8.0 + (i * 3) as f32);
    }

    // pst.generate("pstools_demo.ps".to_string());

}

fn detailed_help() {
    println!("More info and details");
}

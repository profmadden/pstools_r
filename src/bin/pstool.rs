use argh::FromArgs;
/// PSTools
/// A simple library for generating PostScript graphics.  The raw PostScript
/// can then be converted to PDF using a tool like GhostScript.
use pstools::*;
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
    /// alternate grid demo with scaling and translation
    #[argh(switch, short = '2')]
    alt_demo: bool,
    /// detailed help information
    #[argh(switch, short = 'I')]
    info: bool,
    /// version information
    #[argh(switch, short = 'v')]
    version: bool,
}
fn main() {
    println!("PSTools Simplified PostScript generation in Rust");
    let arguments: PSArgs = argh::from_env();

    if arguments.info {
        detailed_help();
        return;
    }

    if arguments.version {
        println!("{}", pstools::pstools_version());
        return;
    }

    let mut pst = PSTool::new();

    if arguments.input.is_some() {
        pst.parse(arguments.input.unwrap()).unwrap();
    }
    // println!("PST has {} events", pst.len());
    if arguments.demo {
        pst.demo();
        // It's possible to push the current scaling and offsets, and then
        // shift the frame of reference.  Note that this does *not* change
        // the PostScript frame of reference.  To compute the bounds of a
        // number of operations, we have to track the actual XY locations of
        // objects, and these are controlled by the scaling and offset.
        // Maybe in the future, do this cleaner?
        pst.add_gsave();
        pst.translate(300.0, 300.0);
        pst.set_scale(0.3);
        pst.demo();
        // pst.pop();
        // pst.push(0.3, 200.0, 0.0);
        // pst.demo();
    }
    if arguments.alt_demo {
        // Black filled box
        pst.set_fill(true);
        pst.add_box(20.0, 20.0, 30.0, 80.0);
        pst.set_fill(false);
        
        pst.add_box(60.0, 20.0, 90.0, 80.0);        
        
        // Green grid at origin
        pst.set_color(0.0, 1.0, 0.0, 1.0);
        grid(&mut pst, 3, 4, 10.0, 12.0);

        

        // Smaller red grid, shifted
        pst.add_gsave();
        pst.set_line_width(6.0);
        pst.add_scale(0.5);
        pst.add_translate(30.0, 40.0);
        pst.set_color(1.0, 0.0, 0.0, 1.0);
        grid(&mut pst, 3, 6, 10.0, 12.0);
        pst.add_grestore();

        // Big blue grid
        pst.add_scale(3.5);
        pst.set_color(0.0, 0.0, 1.0, 1.0);
        grid(&mut pst, 3, 6, 10.0, 12.0);

        // Cyan grid, offset and shrunk a bit
        pst.add_scale(0.6);
        pst.set_line_width(0.5);
        pst.add_translate(36.0, 33.0);
        pst.set_color(0.0, 1.0, 1.0, 1.0);
        grid(&mut pst, 3, 9, 10.0, 12.0);

        pst.set_border(10.0);
    }
    if pst.len() > 0 {
        if arguments.output.is_some() {
            pst.generate(arguments.output.unwrap()).unwrap();
        }

        // if arguments.output.is_some() {
        //     let str = arguments.output.unwrap().clone();
        //     let p = Path::new(&str);
        //     // pst.generate(Some(p)).unwrap();
        // } else {
        //     // pst.generate(None::Path).unwrap();
        // }
        // pst.gentest(Some(Path::new("file.ps"))).unwrap();
        // if arguments.output.is_some() {
        //     pst.generate(arguments.output.unwrap());
        // } else {
        //     pst.generate("".to_string());
        // }
        return;
    } else {
        println!("Use -h for information.");
    }
}

fn detailed_help() {
    println!("More info and details");
}

fn grid(pst: &mut PSTool, r: u32, c: u32, sx: f32, sy: f32) {
    for row in 0..r {
        // Horizontal line
        pst.add_line(0.0, row as f32 * sy, c as f32 * sx, row as f32 * sy);
    }

    for col in 0..c {
        // Horizontal line
        pst.add_line(col as f32 * sx, 0.0, col as f32 * sx, r as f32 * sy);
    }
}

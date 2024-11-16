use std::env;
use pstools_r::Color;
use pstools_r::LBBox;
use pstools_r::Events;
use pstools_r::{*};

fn main() {
    println!("PSTools main!");
    let mut pst = PSTool::new();

    pst.add_color(0.3, 0.4, 0.2, 1.0);
    pst.add_box(5.0, 5.0, 20.0, 30.0);
    pst.add_color(0.8, 0.1, 0.2, 1.0);
    pst.add_box(35.0, 19.0, 7.0, 16.0);
    pst.generate("testfile.ps".to_string());


}
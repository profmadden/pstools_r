use std::env;
use pstools_r::Color;
use pstools_r::LBBox;
use pstools_r::Events;
use pstools_r::{*};

fn main() {
    println!("PSTools main!");

    let mut e = pstools_r::Events {
        e: Vec::new(),
    };

    e.e.push(PSEvent {
        tag: PSTag::C,
        event: PSUnion {
            color:
              Color {
                 r: 3.3,
                 g: 0.045,
                 b: 0.2,
                 a: 1.0,
                fill: true,
              }
            }
    });

    pstools_r::generate(&e);

    let mut pst = PSTool::new();
    pst.add_color(0.3, 0.4, 0.5, 0.2);

}
#[derive(Clone,Copy)]
pub struct LBBox {
    line: bool,
    llx: f32,
    lly: f32,
    urx: f32,
    ury: f32,
}

#[derive(Clone,Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub fill: bool,
}
#[derive(PartialEq)]
pub enum PSTag {B, C}


pub union PSUnion {
    pub line: LBBox,
    pub color: Color,
}

pub struct PSEvent {
    pub tag: PSTag,
    pub event: PSUnion,
}

pub struct Events {
    pub e: Vec<PSEvent>,
}

pub fn generate(events: &Events) {
    for e in &events.e {
        println!("Got event ");
        unsafe {
            if e.tag == PSTag::C {
                let c = e.event.color;
                println!("Color {} {} {} {}", c.r, c.g, c.b, c.a);
            }
            if e.tag == PSTag::B {

            }
        }
    }
}


pub struct PSTool {
    e: Events,
}

impl PSTool {
    pub fn new() -> PSTool {
        PSTool {
            e: Events {
                e: Vec::new(),
            }
        }
    }
    pub fn add_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::C,
                event: PSUnion {
                    color:
                    Color {
                        r: r,
                        g: g,
                        b: b,
                        a: a,
                        fill: true,
                    }
                    }
                });
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn psversion() -> String {
    "PSTools_R version 1.0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

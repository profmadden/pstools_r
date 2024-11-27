pub mod point;
pub mod bbox;

use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone,Copy)]
pub struct LBBox {
    pub line: bool,
    pub llx: f32,
    pub lly: f32,
    pub urx: f32,
    pub ury: f32,
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
pub enum PSTag {B, C, L}


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
    pub fn add_box(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::B,
                event: PSUnion {
                    line: LBBox {
                        line: false,
                        llx: llx,
                        lly: lly,
                        urx: urx,
                        ury: ury,
                    }
                }
            }
        );
    }
    pub fn add_line(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::L,
                event: PSUnion {
                    line: LBBox {
                        line: true,
                        llx: llx,
                        lly: lly,
                        urx: urx,
                        ury: ury,
                    }
                }
            }
        );
    }

    pub fn bbox(&self) -> (f32, f32, f32, f32) {
        // BBox started?
        let mut mark = false;
        let mut llx = 0.0;
        let mut lly = 0.0;
        let mut urx = 0.0;
        let mut ury = 0.0;

        for e in &self.e.e {
            unsafe {
                if e.tag == PSTag::C {

                }
                if e.tag == PSTag::B || e.tag == PSTag::L {
                    if !mark {
                        llx = e.event.line.llx.min(e.event.line.urx);
                        lly = e.event.line.lly.min(e.event.line.lly);
                        urx = e.event.line.llx.max(e.event.line.urx);
                        ury = e.event.line.lly.max(e.event.line.ury);
                        mark = true;
                    } else {
                        llx = llx.min(e.event.line.llx);
                        llx = llx.min(e.event.line.urx);
                        lly = lly.min(e.event.line.llx);
                        lly = lly.min(e.event.line.urx);

                        urx = urx.max(e.event.line.llx);
                        urx = urx.max(e.event.line.urx);
                        ury = ury.max(e.event.line.lly);
                        ury = ury.max(e.event.line.ury);
                    }
                }
            }
        }

        // Expand the bbox slightly
        llx = llx - 10.0;
        lly = lly - 10.0;
        urx = urx + 10.0;
        ury = ury + 10.0;

        (llx, lly, urx, ury)
    }

    pub fn generate(&self, filepath: String) {
        let mut f = File::create(&filepath).unwrap();
        let (origin_x, origin_y, urx, ury) = self.bbox();
        println!("Bounding box {} {}  {} {}", origin_x, origin_y, urx, ury);

        writeln!(&mut f, "%!PS-Adobe-3.0 EPSF-3.0").unwrap();
        writeln!(&mut f, "%%DocumentData: Clean7Bit").unwrap();
        writeln!(&mut f, "%%Origin: {} {}", origin_x, origin_y).unwrap();
        writeln!(&mut f, "%%BoundingBox: {} {} {} {}", origin_x, origin_y, urx - origin_x, ury - origin_y).unwrap();
        writeln!(&mut f, "%%LanguageLevel: 2").unwrap();
        writeln!(&mut f, "%%Pages: 1").unwrap();
        writeln!(&mut f, "%%Page: 1 1").unwrap();
	writeln!(&mut f, "%% gs -o {}.pdf -sDEVICE=pdfwrite -dEPSCrop {}", &filepath, &filepath);
        writeln!(&mut f, "/Courier findfont 15 scalefont setfont").unwrap();
        for e in &self.e.e {
            // println!("Got event ");
            unsafe {
                if e.tag == PSTag::C {
                    let c = e.event.color;
                    writeln!(&mut f, "{} {} {} setrgbcolor", c.r, c.g, c.b).unwrap();
                }
                if e.tag == PSTag::B {
                    writeln!(&mut f, "newpath {} {} moveto", e.event.line.llx, e.event.line.lly).unwrap();
                    writeln!(&mut f, "{} {} lineto", e.event.line.llx, e.event.line.ury).unwrap();
                    writeln!(&mut f, "{} {} lineto", e.event.line.urx, e.event.line.ury).unwrap();
                    writeln!(&mut f, "{} {} lineto", e.event.line.urx, e.event.line.lly).unwrap();    
                    writeln!(&mut f, "{} {} lineto", e.event.line.llx, e.event.line.lly).unwrap();
                    writeln!(&mut f, "closepath fill").unwrap();
                }
                if e.tag == PSTag::L {
                    writeln!(&mut f, "newpath {} {} moveto", e.event.line.llx, e.event.line.lly).unwrap();
                    writeln!(&mut f, "{} {} lineto", e.event.line.urx, e.event.line.ury).unwrap();
                    writeln!(&mut f, "stroke").unwrap();
                }
            }
        }
        writeln!(&mut f, "%%EOF\n").unwrap();

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

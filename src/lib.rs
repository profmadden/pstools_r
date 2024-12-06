pub mod point;
pub mod bbox;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use bbox::BBox;

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

#[derive(Clone,Copy)]
pub struct Fill {
    pub fill: bool,
}

#[derive(Clone,Copy)]
pub struct Text {
    pub text: usize, // Index into the text strings
    pub x: f32,
    pub y: f32,
}

#[derive(Clone,Copy)]
pub struct Font {
    pub scale: f32,
    pub font_name: usize, // Index into the text strings
}

#[derive(Clone,Copy)]
pub struct Comment {
    pub comment: usize, // Index into the text strings
}


#[derive(PartialEq)]
pub enum PSTag {B, C, L, F, T, FN, CM}


pub union PSUnion {
    pub line: LBBox,
    pub color: Color,
    pub fill: Fill,
    pub text: Text,
    pub font: Font,
    pub comment: Comment
}

pub struct PSEvent {
    pub tag: PSTag,
    pub event: PSUnion,
}

pub struct Events {
    pub e: Vec<PSEvent>,
}


pub struct PSTool {
    bbox: BBox,
    border: f32,
    scale: f32,
    e: Events,
    te: Vec<String>,
}

impl PSTool {
    pub fn new() -> PSTool {
        PSTool {
            bbox: BBox {
                valid: false,
                llx: 0.0,
                lly: 0.0,
                urx: 0.0,
                ury: 0.0,
            },
            scale: 1.0,
            border: 0.0,
            e: Events {
                e: Vec::new(),
            },
            te: Vec::new(),
        }
    }
    pub fn add_text(&mut self, x: f32, y: f32, t: String) {
        self.e.e.push(PSEvent{
            tag: PSTag::T,
            event: PSUnion {
                text: Text {
                text: self.te.len(),
                x: x * self.scale,
                y: y * self.scale,
                }
            }
        });
        self.te.push(t);

    }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
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
    pub fn set_fill(&mut self, state: bool) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::F,
                event: PSUnion {
                    fill: Fill {
                    fill: state,
                    },
                }
            }
        );
    }
    pub fn set_font(&mut self, scale: f32, font: String) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::FN,
                event: PSUnion {
                    font: Font {
                        scale: scale,
                        font_name: self.te.len(),
                    }
                }
            }
        );
        self.te.push(font);
    }
    pub fn add_box(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::B,
                event: PSUnion {
                    line: LBBox {
                        line: false,
                        llx: llx * self.scale,
                        lly: lly * self.scale,
                        urx: urx * self.scale,
                        ury: ury * self.scale,
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
                        llx: llx * self.scale,
                        lly: lly * self.scale,
                        urx: urx * self.scale,
                        ury: ury * self.scale,
                    }
                }
            }
        );
    }

    pub fn set_bounds(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.bbox.valid = true;
        self.bbox.llx = llx;
        self.bbox.lly = lly;
        self.bbox.urx = urx;
        self.bbox.ury = ury;
    }

    pub fn set_border(&mut self, border: f32) {
        self.border = border;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn bbox(&self) -> (f32, f32, f32, f32) {
        if self.bbox.valid {
            return (self.bbox.llx, self.bbox.lly, self.bbox.urx, self.bbox.ury);
        }
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
                        lly = lly.min(e.event.line.lly);
                        lly = lly.min(e.event.line.ury);

                        urx = urx.max(e.event.line.llx);
                        urx = urx.max(e.event.line.urx);
                        ury = ury.max(e.event.line.lly);
                        ury = ury.max(e.event.line.ury);
                    }
                }
            }
        }

        // Expand the bbox by the requested border size
        llx = llx - self.border;
        lly = lly - self.border;
        urx = urx + self.border;
        ury = ury + self.border;

        (llx, lly, urx, ury)
    }

    pub fn generate(&self, filepath: String) {
        let mut f = File::create(&filepath).unwrap();
        let (origin_x, origin_y, urx, ury) = self.bbox();
        // println!("Bounding box {} {}  {} {}", origin_x, origin_y, urx, ury);

        writeln!(&mut f, "%!PS-Adobe-3.0 EPSF-3.0").unwrap();
        writeln!(&mut f, "%%DocumentData: Clean7Bit").unwrap();
        writeln!(&mut f, "%%Origin: {} {}", origin_x, origin_y).unwrap();
        writeln!(&mut f, "%%BoundingBox: {} {} {} {}", origin_x, origin_y, urx, ury).unwrap();
        writeln!(&mut f, "%%LanguageLevel: 2").unwrap();
        writeln!(&mut f, "%%Pages: 1").unwrap();
        writeln!(&mut f, "%%Page: 1 1").unwrap();
	    writeln!(&mut f, "%% gs -o {}.pdf -sDEVICE=pdfwrite -dEPSCrop {}", &filepath, &filepath).unwrap();
        writeln!(&mut f, "/Courier findfont 15 scalefont setfont").unwrap();
        let mut fontscale = 15.0;
        let mut fillstate = false;
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
                    if fillstate {
                        writeln!(&mut f, "closepath fill").unwrap();
                    } else {
                        writeln!(&mut f, "stroke").unwrap();
                    }
                }
                if e.tag == PSTag::L {
                    writeln!(&mut f, "newpath {} {} moveto", e.event.line.llx, e.event.line.lly).unwrap();
                    writeln!(&mut f, "{} {} lineto", e.event.line.urx, e.event.line.ury).unwrap();
                    writeln!(&mut f, "stroke").unwrap();
                }
                if e.tag == PSTag::F {
                    fillstate = e.event.fill.fill;
                }
                if e.tag == PSTag::T {
                    writeln!(&mut f, "{} {} moveto", e.event.text.x, e.event.text.y).unwrap();
                    writeln!(&mut f, "({}) show", self.te[e.event.text.text]).unwrap();
                }
                if e.tag == PSTag::FN {
                    writeln!(&mut f, "/{} findfont {} scalefont setfont", self.te[e.event.font.font_name], e.event.font.scale).unwrap();
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

pub mod point;
pub mod bbox;

use std::fs::File;
use std::io::Write;
use bbox::BBox;
use scan_fmt::scan_fmt;

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

#[derive(Clone,Copy)]
pub struct Curve {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub x3: f32,
    pub y3: f32
}


#[derive(PartialEq)]
pub enum PSTag {B, C, L, R, F, T, V, N, FN, CM}


pub union PSUnion {
    pub line: LBBox,
    pub curve: Curve,
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
    pub fn add_comment(&mut self, t: String) {
        self.e.e.push(PSEvent{
            tag: PSTag::N,
            event: PSUnion {
                text: Text {
                    text: self.te.len(),
                    x: 0.0,
                    y: 0.0,
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
    pub fn add_curve(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::V,
                event: PSUnion {
                    curve: Curve {
                        x1: x1 * self.scale,
                        y1: y1 * self.scale,
                        x2: x2 * self.scale,
                        y2: y2 * self.scale,
                        x3: x3 * self.scale,
                        y3: y3 * self.scale,
                    }
                }
            }
        );
    }
    pub fn add_circle(&mut self, x: f32, y: f32, radius: f32) {
        self.e.e.push(
            PSEvent {
                tag: PSTag::R,
                event: PSUnion {
                    line: LBBox {
                        line: false,
                        llx: x * self.scale,
                        lly: y * self.scale,
                        urx: radius * self.scale,
                        ury: 0.0,
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
                if e.tag == PSTag::B || e.tag == PSTag::L || e.tag == PSTag::R {
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

    pub fn len(&self) -> usize {
        self.e.e.len()
    }

    pub fn generate(&self, filepath: String) {
        let mut f;

        // if the file path is empty, just print to standard out
        if filepath.len() == 0 {
            f = Box::new(std::io::stdout()) as Box<dyn Write>;
        } else {
            f = Box::new(File::create(&filepath).unwrap()) as Box<dyn Write>;
        }

        // let mut f = unsafe { std::os::unix::io::from_raw_fd(3); }
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
        writeln!(&mut f, "%% Binghamton PSTool_r PostScript Generator").unwrap();
        writeln!(&mut f, "%% https://github.com/profmadden/pstools_r for more information.").unwrap();
        writeln!(&mut f, "%% ").unwrap();
        writeln!(&mut f, "/Courier findfont 15 scalefont setfont").unwrap();
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
                if e.tag == PSTag::R {
                    if fillstate {
                        writeln!(&mut f, "newpath {} {} {} 0 360 arc fill", e.event.line.llx, e.event.line.lly, e.event.line.urx).unwrap();
                    }
                    else {
                        writeln!(&mut f, "newpath {} {} {} 0 360 arc stroke", e.event.line.llx, e.event.line.lly, e.event.line.urx).unwrap();
                    }
                }
                if e.tag == PSTag::V {
                    writeln!(&mut f, "newpath {} {} moveto {} {} {} {} {} {} curveto stroke",
                    e.event.curve.x1, e.event.curve.y1,
                    e.event.curve.x1, e.event.curve.y1,
                    e.event.curve.x2, e.event.curve.y2,
                    e.event.curve.x3, e.event.curve.y3
                    ).unwrap();
                }
                if e.tag == PSTag::F {
                    fillstate = e.event.fill.fill;
                }
                if e.tag == PSTag::T {
                    writeln!(&mut f, "{} {} moveto", e.event.text.x, e.event.text.y).unwrap();
                    writeln!(&mut f, "({}) show", self.te[e.event.text.text]).unwrap();
                }
                if e.tag == PSTag::N {
                    writeln!(&mut f, "%% {}", self.te[e.event.text.text]).unwrap();
                }
                if e.tag == PSTag::FN {
                    writeln!(&mut f, "/{} findfont {} scalefont setfont", self.te[e.event.font.font_name], e.event.font.scale).unwrap();
                }
            }
        }
        writeln!(&mut f, "%%EOF\n").unwrap();

    }

    pub fn parse(&mut self, filename: &String) {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::with_capacity(32000, f);
        loop {
            let line = getline(&mut reader);
            match line {
                Ok(s) => {
                    // println!("Input line {s}");
                    if let Ok((x1, y1, x2, y2)) = scan_fmt!(&s, "box {} {} {} {}", f32, f32, f32, f32){
                        self.add_box(x1, y1, x2, y2);
                        continue;
                    }
                    if let Ok((x1, y1, x2, y2)) = scan_fmt!(&s, "line {} {} {} {}", f32, f32, f32, f32){
                        self.add_line(x1, y1, x2, y2);
                        continue;
                    }
                    if let Ok((x, y, r)) = scan_fmt!(&s, "circle {} {} {}", f32, f32, f32) {
                        self.add_circle(x, y, r);
                        continue;
                    }
                    if let Ok((r, g, b)) = scan_fmt!(&s, "color {} {} {}", f32, f32, f32) {
                        self.set_color(r, g, b, 1.0);
                        continue;
                    }
                    if let Ok(fill) = scan_fmt!(&s, "fill {}", usize) {
                        self.set_fill(fill != 0);
                        continue;
                    }
                    if let Ok((x1, y1, x2, y2, x3, y3)) = scan_fmt!(&s, "curve {} {} {} {} {} {}", f32, f32, f32, f32, f32, f32) {
                        self.add_curve(x1, y1, x2, y2, x3, y3);
                        continue;
                    }
                    if let Ok((scale, font)) = scan_fmt!(&s, "font {} {}", f32, String){
                        self.set_font(scale, font);
                        continue;
                    }
                    if let Ok((x, y, str))= scan_fmt!(&s, "text {} {} {}", f32, f32, String) {
                        self.add_text(x, y, str);
                        continue;
                    }
                    if let Ok(str) = scan_fmt!(&s, "comment {}", String) {
                        self.add_comment(str);
                        continue;
                    }
                },
                _ => {return;}
            }
        }
    }
}

use std::io::{BufRead,BufReader};
use std::io::{Error,ErrorKind};

fn getline(reader: &mut BufReader<File>) -> std::io::Result<String> {
    loop {
        let mut line = String::new();
        let _len = reader.read_line(&mut line).unwrap();
        // println!("Read in {} bytes, line {}", _len, line);

        if _len == 0 {
            return std::result::Result::Err(Error::new(ErrorKind::Other, "end of file"));
        }

        if line.starts_with("#") {
            // println!("Skip comment.");
            continue;
        }

        if _len == 1 {
            continue;
        }

        return Ok(line.trim().to_string());
    }
}


pub fn psversion() -> String {
    "PSTools_R version 1.0".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}

// PSTools PostScript drawing library
// Patrick H. Madden/pmadden@binghamton.edu
// PostScript is cool, yo!  And sometimes you need to draw some
// simple figures, and don't want to hassle with a complex API.
pub mod bbox;
pub mod point;

use bbox::BBox;
use scan_fmt::scan_fmt;
use std::fs::File;
use std::io::Write;
use std::io::Result;
// use std::fs::Path;
use std::path::Path;

#[derive(Clone, Copy)]
struct LBBox {
    // pub line: bool,
    pub llx: f32,
    pub lly: f32,
    pub urx: f32,
    pub ury: f32,
}

// Color in PostScript land is just RGB, but I'm keeping
// an alpha channel here -- might adapt the code later to
// generate a PNG file (where alpha could matter).
#[derive(Clone, Copy)]
struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub _a: f32,
}

#[derive(Clone, Copy)]
struct Fill {
    pub fill: bool,
}

// Using a union to represent different types of events;
// this doesn't work if there are objects in the structs, so
// I'm using a separate vector to hold a string, and then
// the entry in the event vector just has an index into the
// string vector.  There's probably a better Rust way to do
// this, but as an old C dog trying to learn a new trick,
// this is what I came up with.
#[derive(Clone, Copy)]
struct Text {
    pub text: usize, // Index into the text strings
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

#[derive(Clone, Copy)]
struct Font {
    pub scale: f32,
    pub font_name: usize, // Index into the text strings
}

#[derive(Clone, Copy)]
struct Curve {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub x3: f32,
    pub y3: f32,
}

// Events are stored in a vector, with a union structure.  To
// decode the union, we use a tag - B for Box, C for color,
// L for line, R for cirRcle, F for fill, T for text,
// V for curVe, N for note(comment), FN for font name.
// The unions do not have a String field -- for the events
// where a String is required, we use an index into a vector
// of strings.
#[derive(PartialEq)]
enum PSTag {
    B, // Box
    C, // Color
    L,  // Line
    R, // ciRcle
    F, // Fill
    T, // Text
    V, // curVe
    N, // Note/Comment
    FN, // Font
    P, // Raw PostScript
}

union PSUnion {
    line: LBBox,
    curve: Curve,
    color: Color,
    fill: Fill,
    text: Text,
    font: Font,
    // comment: Comment,
}

struct PSEvent {
    tag: PSTag,
    event: PSUnion,
}

/// The PSTool structure records a series of PostScript events -- drawing of
/// lines, boxes, circles, color changes, text, and so on.  A bounding box of the
/// events is computed when the generate function is called, with the events being
/// output to the specified file as raw PostScript commands.
/// <br>
/// By default, line widths are 1 point.  A scale factor can be set (with each
/// event scaled by that amount when it is added).
pub struct PSTool {
    bbox: BBox,
    border: f32,
    scale: f32,
    events: Vec<PSEvent>,
    te: Vec<String>,
    text_x: f32,
    text_y: f32,
    text_line_space: f32,
    notes: Vec<String>,
}

impl PSTool {
    /// Create a new PSTool instance
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
            events: Vec::new(),
            te: Vec::new(),
            text_x: 0.0,
            text_y: 0.0,
            text_line_space: 12.0,
            notes: Vec::new(),
        }
    }

    /// Add an axis-aligned box to the generated output.  The box will
    /// use the current fill status, and selected color.
    pub fn add_box(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.events.push(PSEvent {
            tag: PSTag::B,
            event: PSUnion {
                line: LBBox {
                    // line: false,
                    llx: llx * self.scale,
                    lly: lly * self.scale,
                    urx: urx * self.scale,
                    ury: ury * self.scale,
                },
            },
        });
    }

    /// Add a line between the indicated coordinates, using the current
    /// selected color.
    pub fn add_line(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.events.push(PSEvent {
            tag: PSTag::L,
            event: PSUnion {
                line: LBBox {
                    // line: true,
                    llx: llx * self.scale,
                    lly: lly * self.scale,
                    urx: urx * self.scale,
                    ury: ury * self.scale,
                },
            },
        });
    }
    /// Add a circle at the indicated coordinates and radius, using the
    /// current fill status and color.
    pub fn add_circle(&mut self, x: f32, y: f32, radius: f32) {
        self.events.push(PSEvent {
            tag: PSTag::R,
            event: PSUnion {
                line: LBBox {
                    // line: false,
                    llx: x * self.scale,
                    lly: y * self.scale,
                    urx: radius * self.scale,
                    ury: 0.0,
                },
            },
        });
    }
    /// Add a curve, using a start, mid, and end point.  PostScript supports
    /// Bezier curves; a curve can be helpful in showing a connection where
    /// co-linear connections might often overlap.
    pub fn add_curve(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.events.push(PSEvent {
            tag: PSTag::V,
            event: PSUnion {
                curve: Curve {
                    x1: x1 * self.scale,
                    y1: y1 * self.scale,
                    x2: x2 * self.scale,
                    y2: y2 * self.scale,
                    x3: x3 * self.scale,
                    y3: y3 * self.scale,
                },
            },
        });
    }

    /// Adds raw PostScript commands to the instruction stream.
    /// Can be used to get full access to PostScript functionality
    /// (gsave/grestore, translation, scaling, and so on).  Note that
    /// the bounding box of the output will not correctly track the
    /// graphic elements introduced with raw PostScript.
    pub fn add_postscript(&mut self, t: String) {
        self.events.push(PSEvent {
            tag: PSTag::P,
            event: PSUnion {
                text: Text {
                    text: self.te.len(),
                    x: 0.0,
                    y: 0.0,
                    angle: 0.0,
                },
            },
        });
        self.te.push(t);
    }

    /// Adds text onto the display at the specified coordintes. The currently
    /// selected color, font, and font size will be utilized.  Note that
    /// currently the String is not character-escaped; close parentheses
    /// may not render correctly (or cause invalid PostScript generation).
    pub fn add_text(&mut self, x: f32, y: f32, t: String) {
        self.events.push(PSEvent {
            tag: PSTag::T,
            event: PSUnion {
                text: Text {
                    text: self.te.len(),
                    x: x * self.scale,
                    y: y * self.scale,
                    angle: 0.0,
                },
            },
        });
        self.te.push(t);
    }
    /// Adds text onto the display at the specified coordintes. The currently
    /// selected color, font, and font size will be utilized.  Note that
    /// currently the String is not character-escaped; close parentheses
    /// may not render correctly (or cause invalid PostScript generation).
    pub fn add_text_rotated(&mut self, x: f32, y: f32, angle: f32, t: String) {
        self.events.push(PSEvent {
            tag: PSTag::T,
            event: PSUnion {
                text: Text {
                    text: self.te.len(),
                    x: x * self.scale,
                    y: y * self.scale,
                    angle: angle,
                },
            },
        });
        self.te.push(t);
    }

    /// Sets the location for text lines (and auto-increments the
    /// line position with each add_text_nl call).
    pub fn set_text_ln(&mut self, x: f32, y: f32) {
        self.text_x = x;
        self.text_y = y;
    }

    /// Adds a string at the current text line location, then increments
    /// to the next position
    pub fn add_text_ln(&mut self, t: String) {
        // May want to add character escapes, and support multiple line strings
        // Here's how they do URL encoding. Looks like it examines one letter at
        // a time, with characters encoded into u8. 
        // https://docs.rs/urlencoding/2.1.3/src/urlencoding/enc.rs.html#72-74
        self.add_text(self.text_x, self.text_y, t);
        self.text_y = self.text_y - self.text_line_space;
    }

    /// Adds commented text to the PostScript output; while this
    /// will not cause visual changes to the generated PostScript, it
    /// can be helpful for annotating a PostScript file with information
    /// such as configuration parameters or command line arguments for
    /// a software tool.  The comment will be embedded in the PostScript
    /// file in sequence with other events -- if there's something specific
    /// being drawn, it may be helpful to have the comment adjacent to
    /// the PostScript commands in the output file.
    /// PSTool also supports notes, which are added to the start of the
    /// output file.
    pub fn add_comment(&mut self, t: String) {
        self.events.push(PSEvent {
            tag: PSTag::N,
            event: PSUnion {
                text: Text {
                    text: self.te.len(),
                    x: 0.0,
                    y: 0.0,
                    angle: 0.0,
                },
            },
        });
        self.te.push(t);
    }
    pub fn add_note(&mut self, s: String) {
        self.notes.push(s);
    }
    /// Generates a very simple two-dimensional chart, using floating
    /// point numbers from the data vector.  The size of the chart is
    /// specified by the bounding coordinates.  The data in the input
    /// vector is scaled so that the range of values fills the vertical
    /// span of the chart.  The chart is written with the currently
    /// specified color.  If the input max and min are equal, the range
    /// is determined by the data.  Otherwise, the supplied max and min will
    /// be used (with these limits clamping the data).
    pub fn chart(
        &mut self,
        data: Vec<f32>,
        min: f32,
        max: f32,
        llx: f32,
        lly: f32,
        urx: f32,
        ury: f32,
    ) {
        if data.len() == 0 || llx == urx || lly == ury {
            return;
        }

        let mut chart_min;
        let mut chart_max;
        if min != max {
            chart_min = min;
            chart_max = max;
        } else {
            chart_min = data[0];
            chart_max = data[0];
            for v in &data {
                if chart_min > *v {
                    chart_min = *v;
                }
                if chart_max < *v {
                    chart_max = *v;
                }
            }
        }
        self.add_box(llx, lly, urx, ury);
        let dx = urx - llx;
        let dy = ury - lly;
        let range = chart_max - chart_min;
        let mut prior_x = 0.0;
        let mut prior_y = 0.0;

        for i in 0..data.len() {
            let x = llx + dx * ((i as f32) / (data.len() as f32));
            let mut y = lly + dy * ((data[i] - chart_min) / range);
            // Clamp the range
            if y > lly + dy {
                y = lly + dy;
            }
            if y < lly {
                y = lly;
            }
            if i != 0 {
                self.add_line(prior_x, prior_y, x, y);
            }
            prior_x = x;
            prior_y = y;
        }
    }

    /// Sets the color for object rendering, using Red/Green/Blue
    /// hues, where each of these values is in the range of 0.0-1.0.
    /// The library currently also supports n alpha color channel,
    /// which is not supported by PostScript.  Future versions of this
    /// library may add support for PNG file generation (where alpha
    /// would be relevant).
    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.events.push(PSEvent {
            tag: PSTag::C,
            event: PSUnion {
                color: Color {
                    r,
                    g,
                    b,
                    _a: a,
                },
            },
        });
    }

    /// Sets the state of rectangle and circle filling; true causes
    /// a filled object, false only draws the outline.
    pub fn set_fill(&mut self, state: bool) {
        self.events.push(PSEvent {
            tag: PSTag::F,
            event: PSUnion {
                fill: Fill { fill: state },
            },
        });
    }

    /// The generated PostScript has a bounding box (determined by the
    /// coordinates of boxes and lines).  When converting to PDF, the
    /// resulting file wraps the bounding box tightly.  To add additional
    /// space around a figure, a border can be added (effectively expanding
    /// the size of the bounding box used in PDF generation).
    pub fn set_border(&mut self, border: f32) {
        self.border = border;
    }
    /// The bounds for a figure can be set explicitly -- the fixed bounding
    /// box can be used to trim a figure to only an area of interest.
    pub fn set_bounds(&mut self, llx: f32, lly: f32, urx: f32, ury: f32) {
        self.bbox.valid = true;
        self.bbox.llx = llx;
        self.bbox.lly = lly;
        self.bbox.urx = urx;
        self.bbox.ury = ury;
    }

    /// Lines generated are one point wide by default; the scale of all
    /// graphic elements can be adjusted if needed, so that the line widths appear
    /// reasonable.  Set the scale prior to adding elements; the scaling
    /// factor is applied to coordinates as these elements are added.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    /// The font size and scale can be set; the scale of a font is normally
    /// given in points.  The supported fonts may depend on a PostScript
    /// engine, but Times-Roman, Helvetica, and Courier are generally
    /// available.
    pub fn set_font(&mut self, scale: f32, font: String) {
        self.events.push(PSEvent {
            tag: PSTag::FN,
            event: PSUnion {
                font: Font {
                    scale: scale,
                    font_name: self.te.len(),
                },
            },
        });
        self.te.push(font);
        self.text_line_space = scale * 1.1;
    }

    /// Returns the bounding box of elements that have been added.
    /// The bounding box does not track text entries -- only lines, and boxes.
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

        for e in &self.events {
            unsafe {
                if e.tag == PSTag::C {}
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
    /// Returns the length of the event vector -- the number of objects
    /// that have been added.  If no events have been added to a PSTool,
    /// PostScript output will not be generated.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn gentest2<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        let mut f = File::open(path)?;

        Ok(())
    }

    pub fn gentest(&self, filepath: Option<&Path>) -> std::io::Result<Box<Error>> {
        let mut f;

        // if the file path is empty, just print to standard out
        if filepath.is_none() {
            f = Box::new(std::io::stdout()) as Box<dyn Write>;
        } else {
            f = Box::new(File::create(filepath.unwrap()).unwrap()) as Box<dyn Write>;
        }

        // let mut f = unsafe { std::os::unix::io::from_raw_fd(3); }
        let (origin_x, origin_y, urx, ury) = self.bbox();
        // println!("Bounding box {} {}  {} {}", origin_x, origin_y, urx, ury);

        writeln!(&mut f, "%!PS-Adobe-3.0 EPSF-3.0")?;
        writeln!(&mut f, "%%DocumentData: Clean7Bit").unwrap();
        if self.bbox.valid {
            writeln!(&mut f, "%%Origin: {} {}", self.bbox.llx, self.bbox.lly).unwrap();
            writeln!(
                &mut f,
                "%%BoundingBox: {} {} {} {}",
                self.bbox.llx, self.bbox.lly, self.bbox.urx, self.bbox.ury
            )?;
        } else {
            writeln!(&mut f, "%%Origin: {} {}", origin_x, origin_y).unwrap();
            writeln!(
                &mut f,
                "%%BoundingBox: {} {} {} {}",
                origin_x, origin_y, urx, ury
            )?;
        }
        writeln!(&mut f, "%%LanguageLevel: 2").unwrap();
        writeln!(&mut f, "%%Pages: 1").unwrap();
        writeln!(&mut f, "%%Page: 1 1").unwrap();
        if filepath.is_some() {
            let fp = filepath.unwrap();
            let fppdf = filepath.unwrap().with_extension("pdf");
            writeln!(&mut f, "%% gs -o {} -sDEVICE=pdfwrite -dEPSCrop {}", fp.display(), fppdf.display())?;
        }

        Ok()

    }
    /// Generates simple PostScript to express the objects that have
    /// been added.  The filepath should be the name of a file to store
    /// the PostScript in.  If this string is zero length, output will
    /// be sent to standard out.
    pub fn generate<P: AsRef<Path>>(&self, filepath: Option<P>) -> Result<usize, Box<Error>> {
        let mut f;

        // if the file path is empty, just print to standard out
        if filepath.is_none() {
            f = Box::new(std::io::stdout()) as Box<dyn Write>;
        } else {
            f = Box::new(File::create(filepath.unwrap()).unwrap()) as Box<dyn Write>;
        }

        // let mut f = unsafe { std::os::unix::io::from_raw_fd(3); }
        let (origin_x, origin_y, urx, ury) = self.bbox();
        // println!("Bounding box {} {}  {} {}", origin_x, origin_y, urx, ury);

        writeln!(&mut f, "%!PS-Adobe-3.0 EPSF-3.0")?;
        writeln!(&mut f, "%%DocumentData: Clean7Bit").unwrap();
        if self.bbox.valid {
            writeln!(&mut f, "%%Origin: {} {}", self.bbox.llx, self.bbox.lly).unwrap();
            writeln!(
                &mut f,
                "%%BoundingBox: {} {} {} {}",
                self.bbox.llx, self.bbox.lly, self.bbox.urx, self.bbox.ury
            )?;
        } else {
            writeln!(&mut f, "%%Origin: {} {}", origin_x, origin_y).unwrap();
            writeln!(
                &mut f,
                "%%BoundingBox: {} {} {} {}",
                origin_x, origin_y, urx, ury
            )?;
        }
        writeln!(&mut f, "%%LanguageLevel: 2").unwrap();
        writeln!(&mut f, "%%Pages: 1").unwrap();
        writeln!(&mut f, "%%Page: 1 1").unwrap();
        writeln!(&mut f, "%% gs -o filename.pdf -sDEVICE=pdfwrite -dEPSCrop filename.ps").unwrap();
        // if filepath.is_some() {
        //     let fp: AsRef<Path> = &filepath.unwrap();
        //     writeln!(
        //         &mut f,
        //         "%% gs -o {}.pdf -sDEVICE=pdfwrite -dEPSCrop {}",
        //         fp.display(), filepath.unwrap().display()
        //     )
        //     .unwrap();
        // }
        writeln!(&mut f, "%% Binghamton PSTools PostScript Generator").unwrap();
        writeln!(
            &mut f,
            "%% https://github.com/profmadden/pstools_r for more information."
        )
        .unwrap();
        writeln!(&mut f, "%% ").unwrap();
        for s in &self.notes {
            writeln!(
                &mut f,
                "%% {}", s,
            ).unwrap();
        }
        writeln!(&mut f, "/Courier findfont 15 scalefont setfont").unwrap();
        let mut fillstate = false;
        for e in &self.events {
            // println!("Got event ");
            unsafe {
                if e.tag == PSTag::C {
                    let c = e.event.color;
                    writeln!(&mut f, "{} {} {} setrgbcolor", c.r, c.g, c.b).unwrap();
                }
                if e.tag == PSTag::B {
                    writeln!(
                        &mut f,
                        "newpath {} {} moveto",
                        e.event.line.llx, e.event.line.lly
                    )
                    .unwrap();
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
                    writeln!(
                        &mut f,
                        "newpath {} {} moveto",
                        e.event.line.llx, e.event.line.lly
                    )
                    .unwrap();
                    writeln!(&mut f, "{} {} lineto", e.event.line.urx, e.event.line.ury).unwrap();
                    writeln!(&mut f, "stroke").unwrap();
                }
                if e.tag == PSTag::R {
                    if fillstate {
                        writeln!(
                            &mut f,
                            "newpath {} {} {} 0 360 arc fill",
                            e.event.line.llx, e.event.line.lly, e.event.line.urx
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            &mut f,
                            "newpath {} {} {} 0 360 arc stroke",
                            e.event.line.llx, e.event.line.lly, e.event.line.urx
                        )
                        .unwrap();
                    }
                }
                if e.tag == PSTag::V {
                    writeln!(
                        &mut f,
                        "newpath {} {} moveto {} {} {} {} {} {} curveto stroke",
                        e.event.curve.x1,
                        e.event.curve.y1,
                        e.event.curve.x1,
                        e.event.curve.y1,
                        e.event.curve.x2,
                        e.event.curve.y2,
                        e.event.curve.x3,
                        e.event.curve.y3
                    )
                    .unwrap();
                }
                if e.tag == PSTag::F {
                    fillstate = e.event.fill.fill;
                }
                if e.tag == PSTag::T {
                    if e.event.text.angle != 0.0 {
                        writeln!(&mut f, "gsave {} {} translate {} rotate 0 0 moveto", e.event.text.x, e.event.text.y, e.event.text.angle).unwrap();
                        writeln!(&mut f, "({}) show grestore", self.te[e.event.text.text]).unwrap();
                    } else {
                        writeln!(&mut f, "{} {} moveto", e.event.text.x, e.event.text.y).unwrap();
                        writeln!(&mut f, "({}) show", self.te[e.event.text.text]).unwrap();
                    }
                }
                if e.tag == PSTag::N {
                    writeln!(&mut f, "%% {}", self.te[e.event.text.text]).unwrap();
                }
                if e.tag == PSTag::FN {
                    writeln!(
                        &mut f,
                        "/{} findfont {} scalefont setfont",
                        self.te[e.event.font.font_name], e.event.font.scale
                    )
                    .unwrap();
                }
                if e.tag == PSTag::P {
                    writeln!(
                        &mut f,
                        "{}",
                        self.te[e.event.text.text]).unwrap();
                }
            }
        }
        writeln!(&mut f, "%%EOF\n").unwrap();

        Ok(0)
    }

    /// Simple text file commands can be parsed, and converted into PostScript.  There should be one command
    /// per line.  Blank lines, and lines starting with a hash mark are ignored.
    pub fn parse(&mut self, filename: &String) {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::with_capacity(32000, f);
        loop {
            let line = getline(&mut reader);
            match line {
                Ok(s) => {
                    // println!("Input line {s}");
                    if let Ok((x1, y1, x2, y2)) =
                        scan_fmt!(&s, "box {} {} {} {}", f32, f32, f32, f32)
                    {
                        self.add_box(x1, y1, x2, y2);
                        continue;
                    }
                    if let Ok((x1, y1, x2, y2)) =
                        scan_fmt!(&s, "line {} {} {} {}", f32, f32, f32, f32)
                    {
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
                    if let Ok((x1, y1, x2, y2, x3, y3)) =
                        scan_fmt!(&s, "curve {} {} {} {} {} {}", f32, f32, f32, f32, f32, f32)
                    {
                        self.add_curve(x1, y1, x2, y2, x3, y3);
                        continue;
                    }
                    if let Ok((scale, font)) = scan_fmt!(&s, "font {} {}", f32, String) {
                        self.set_font(scale, font);
                        continue;
                    }
                    if let Ok((x, y, str)) = scan_fmt!(&s, "text {} {} {}", f32, f32, String) {
                        self.add_text(x, y, str);
                        continue;
                    }
                    if let Ok(str) = scan_fmt!(&s, "comment {}", String) {
                        self.add_comment(str);
                        continue;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    /// This routine adds a number of events to the PSTool object, as a means
    /// to demonstrate how each of these elements is used, and how they would
    /// appear in the generated PostScript/PDF.
    pub fn demo(&mut self) {
        self.add_note("This is a note -- placed towards the start of the PS file.".to_string());



        self.set_fill(true);
        self.set_color(0.3, 0.4, 0.2, 1.0);
        self.add_box(5.0, 5.0, 20.0, 30.0);
        self.set_color(0.8, 0.1, 0.2, 1.0);
        self.add_box(35.0, 19.0, 7.0, 16.0);
        self.add_line(33.2, 44.1, 8.7, 5.5);
        self.add_comment("This text is inserted directly into the PS file".to_string());
        self.add_comment("The location of this comment is after some earlier PS events".to_string());

        self.set_color(0.1, 0.1, 0.8, 1.0);
        self.set_fill(false);
        self.add_box(50.0, 50.0, 60.0, 60.0);

        self.set_color(0.1, 0.1, 0.8, 1.0);
        self.set_fill(true);
        self.add_box(70.0, 70.0, 80.0, 80.0);

        self.set_color(0.0, 0.0, 0.0, 1.0);
        self.add_text(10.0, 10.0, "1 Hello World".to_string());

        self.set_color(0.0, 0.0, 1.0, 1.0);
        self.set_font(20.0, "Times-Roman".to_string());
        self.add_text(10.0, 30.0, "2 Hello World".to_string());

        self.set_font(20.0, "Helvetica".to_string());
        self.set_color(0.0, 1.0, 0.0, 1.0);
        self.add_text(10.0, 50.0, "3 Hello World".to_string());

        self.set_border(10.0);

        self.add_circle(100.0, 40.0, 8.0);
        self.set_color(1.0, 0.0, 0.0, 1.0);
        self.set_fill(false);
        self.add_circle(120.0, 80.0, 30.0);

        self.set_color(1.0, 0.0, 0.0, 1.0);
        
        for i in 1..10 {
            self.set_color(0.6, i as f32 * 0.1, i as f32 * 0.1, 1.0);
            self.add_curve(
                4.0,
                150.0,
                90.0,
                30.0 + (i * 6) as f32,
                150.0,
                8.0 + (i * 11) as f32,
            );
        }

        self.add_postscript("gsave 100 100 translate 20 rotate".to_string());
        self.set_fill(true);
        self.add_box(-5.0, 15.0, 100.0, -30.0);
        self.set_color(0.0, 0.0, 0.0, 1.0);
        self.set_fill(false);
        self.set_font(5.0, "Helvetica-Bold".to_string());
        self.set_text_ln(0.0, 0.0);
        self.add_text_ln("Raw PostScript canvas translation and rotation".to_string());
        self.add_text_ln("add_text_ln will advance to new lines".to_string());
        self.add_text_ln("Automatic".to_string());
        self.add_text_ln("Line".to_string());
        self.add_text_ln("Advancing".to_string());

        self.add_postscript("grestore".to_string());

        // Charting operates on a single vector of f32s.
        let mut data = Vec::new();
        for i in 0..100 {
            data.push(((i as f32) / 10.0).sin());
        }
        self.set_color(0.3, 0.3, 0.3, 1.0);
        self.chart(data, -1.2, 1.2, 100.0, 100.0, 300.0, 200.0);

        self.set_color(0.1, 0.1, 1.0, 1.0);
        self.set_font(15.0, "Courier".to_string());
        self.add_text(30.0, 160.0, pstools_version());

        self.add_text_rotated(15.0, 60.0, 90.0, "Rotated text".to_string());

        self.set_bounds(-5.0, -10.0, 210.0, 208.0);
    }
}

use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

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

/// Returns information string for the installed version.
pub fn pstools_version() -> String {
    "PSTools version 0.1".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

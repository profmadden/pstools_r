use std::fmt;
use crate::point;


#[derive(Copy, Clone)]
pub struct BBox {
    pub valid: bool,
    pub llx: f32,
    pub lly: f32,
    pub urx: f32,
    pub ury: f32,
}


impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({}, {}) - ({}, {})]:{}", self.llx, self.lly, self.urx, self.ury, self.area())
    }
}

impl BBox {
    pub fn new() -> BBox {
        BBox {
            valid: false,
            llx: 0.0,
            lly: 0.0,
            urx: 0.0,
            ury: 0.0,
        }
    }
    pub fn addpoint(&mut self, px: f32, py: f32) {
        if !self.valid {
            self.llx = px;
            self.lly = py;
            self.urx = px;
            self.ury = py;
            self.valid = true;
        } else {
            self.llx = self.llx.min(px);
            self.lly = self.lly.min(py);
            self.urx = self.urx.max(px);
            self.ury = self.ury.max(py);
        }
    }
    pub fn area(&self) -> f32 {
        (self.urx - self.llx)*(self.ury-self.lly)
    }
    pub fn dx(&self) -> f32 {
        self.urx - self.llx
    }
    pub fn dy(&self) -> f32 {
        self.ury - self.lly
    }
    pub fn center(&self) -> point::Point {
        point::Point {
            x: (self.urx + self.llx)/ 2.0,
            y: (self.ury + self.lly)/ 2.0,
        }
    }
    pub fn expand(&mut self, other: &BBox) {
        self.llx = self.llx.min(other.llx);
        self.lly = self.lly.min(other.lly);
        self.urx = self.urx.max(other.urx);
        self.ury = self.ury.max(other.ury);
    }
     // Functions to split a region, giving two new regions --
     pub fn split_h(&self, bias: f32) -> (BBox, BBox) {
        let mut bottom = *self;
        let mut top = *self;
        let dy = self.dy();
        bottom.ury = bottom.lly + bias*dy;
        top.lly = bottom.ury;
        (bottom, top)
     }
     pub fn split_v(&self, bias: f32) -> (BBox, BBox) {
        let mut left = *self;
        let mut right = *self;
        let dx = self.dx();
        left.urx = left.llx + bias*dx;
        right.llx = left.urx;
        (left, right)
     }
        
}

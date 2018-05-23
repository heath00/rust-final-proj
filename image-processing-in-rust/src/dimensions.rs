pub struct ImageInfo {
    data: Vec<u8>,
    width: u32,
    height: u32,

}

impl ImageInfo {

    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        ImageInfo {
            data,
            width,
            height,
        }
    }

    pub fn data(&self) -> Vec<u8> {
        self.data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

}

// Contains info on the location of a face:  The rectangular box the face is in, and the box's orientation
pub struct FaceLoc {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rotation: f64,
    skew: f64,
}

impl FaceLoc {

    pub fn new(x: u32, y: u32, w: u32, h: u32, r: f64, p: f64) -> Self {
        FaceLoc{
            x: x,
            y: y,
            width: w,
            height: h,
            rotation: r,
            skew: p,
        }
    }

    pub fn new_blank() -> Self {
        FaceLoc{
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            rotation: 0.0,
            skew: 0.0,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

}



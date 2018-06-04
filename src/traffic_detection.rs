extern crate image;
use image::{GenericImage, Pixel, DynamicImage};

pub fn count_example() {
    let img = image::open("out_images\\sobel.jpg").unwrap();

    let mut new_blob_test = ThreeLaneBlob::new(img, (0,0), (323, 63), (0,0));
    let mynewim = new_blob_test.create_outlined_image();

}

pub fn count_cars(filename: &str, areas_of_interest: &Vec<VehicleBlock>) {

}

#[derive(Debug, Copy, Clone)]
pub struct VehicleBlock {
    top_y_ind: u32,
    width: u32,
    height: u32,
    lane: u8,
}

impl VehicleBlock {
    pub fn new(top_y_ind: u32, width: u32, height: u32, lane: u8) -> Self {
        VehicleBlock {
            top_y_ind,
            width,
            height,
            lane, // will either be 0, 1, or 2 numbered from left to right
        }
    }

    pub fn get_lane(&self) -> u8 {self.lane}

    pub fn get_top_y(&self) -> u32 {self.top_y_ind}
}

pub struct ThreeLaneBlob {
    im: DynamicImage,
    left: (u32, u32),
    middle: (u32, u32),
    right: (u32, u32),
    blocks: Vec<VehicleBlock>,
}

impl ThreeLaneBlob {
    pub fn new(im: DynamicImage, left: (u32, u32), middle: (u32, u32), right: (u32, u32)) -> Self {
        ThreeLaneBlob {
            im,
            left,
            middle,
            right,
            blocks: Vec::new(),
        }
    }

    fn find_car_middle(&mut self) {
        let mut x_ind = self.middle.0;
        let mut y_ind = self.middle.1;

        let mut i = y_ind.clone();



        while y_ind < 700 {

            // search a wide window, not just a line
            for j in (x_ind - 10)..(x_ind + 11) {
                let this_px = self.im.get_pixel(j, y_ind).channels()[0];


                // we have the first edge for a car
                if this_px == 255 {

                    let top_y_ind = y_ind.clone();
                    let h = self.find_height(x_ind, &mut y_ind);
                    println!("{}", h);

                    // now we know the height of this car
                    // need to basically do the same thing but with it's width
                    let mdpt_y = top_y_ind + (h / 2);

                    println!("calling width with: {}, {}", x_ind, mdpt_y);
                    let w = self.find_width(x_ind, mdpt_y);
                    self.blocks.push(VehicleBlock::new(top_y_ind, w, h, 1));
                }

            }
            y_ind += 1;
        }



        println!("{:?}", self.blocks);
    }

    pub fn create_outlined_image(&mut self) -> DynamicImage {
        let mut new_im = self.im.clone();
        self.find_car_middle();

        for block in self.blocks.clone() {
            // figure out what x index we need to deal with
            // depending on which lane we're examining
            let x_ind = match block.get_lane() {
                0 => self.left.0,
                1 => self.middle.0,
                2 => self.right.0,
                x => panic!("unexpected lane"),
            };

            let this_top_y = block.get_top_y();

            // top line
            for x in (x_ind - (width / 2))..(x_ind + (width / 2)) {
                let this_px = new_im.get_pixel(x, block.get_top_y);
            }



        }

        new_im


    }

    fn find_height(&self, x_ind: u32, y_ind: &mut u32) -> u32{
        let top_ind_y = y_ind.clone(); // the top of the car
        let mut bot_ind_y = y_ind.clone(); // initialized as top of car, will become bottom
        let mut car_here = true; // there is a car here

        println!("top_ind_y = {}", top_ind_y);

        while car_here {
            car_here = false;

            // now search a 20x30 window for a white pixel
            // if there is a white pixel, we set it as our bottom y index and continue
            for window_y in bot_ind_y + 1..(bot_ind_y + 31) {
                for window_x in (x_ind - 10)..(x_ind + 11) {

                    // if this pixel is white, we're still somewhere in the middle of the car
                    // we then want to restart the window searching down from this pixel
                    if self.im.get_pixel(window_x, window_y).channels()[0] > 0 {
                        car_here = true;
                        bot_ind_y = window_y;
                        *y_ind = bot_ind_y;
                        break;
                    }
                }

                // we found a car in this column of pixels,
                // let's now break and search the window below this.
                if car_here == true {break;}
            }
        }
        bot_ind_y - top_ind_y
    }

    fn find_width(&self, x_ind: u32, y_ind: u32) -> u32{

        // begin by sweeping left with a search window
        // from the point that we know to be the middle height
        // point of the car
        let mut leftmost_x_ind = x_ind.clone();
        let mut car_here=  true;
        while car_here {
            car_here = false;


            // create 10 x 20 window,
            // sweep along x first
            for window_y in (y_ind - 10)..(y_ind + 11) {
                for window_x in (leftmost_x_ind - 11)..(leftmost_x_ind - 1) {
                    if self.im.get_pixel(window_x, window_y).channels()[0] > 0 {
                        car_here = true;
                        leftmost_x_ind = window_x;
                        break;
                    }
                }
                if car_here {break;}
            }
        }
        (x_ind - leftmost_x_ind) * 2
    }


}

pub struct SearchVector {
    start: (u32, u32),
    end: (u32, u32),
    width: u32,
}



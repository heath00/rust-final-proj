extern crate image;
use image::{GenericImage, Pixel, DynamicImage};
use std::fs::{read_dir};


pub fn count_example(inp: &str, out: &str) {

    let img = image::open(inp).unwrap();

    let left_sv = SearchVector::new((250, 50), (45, 615), 15);
    let mid_sv = SearchVector::new((300, 50), (300, 650), 20);
    let right_sv = SearchVector::new((350, 50), (570, 650), 15);

    let left_sv_angle2 = SearchVector::new((515, 310), (170, 610), 13);
    let mid_sv_angle2 = SearchVector::new((585, 310), (330, 650), 15);
    let right_sv_angle2 = SearchVector::new((650, 310), (500, 650), 16);

    let mut new_blob_test = ThreeLaneBlob::new(img, left_sv_angle2, mid_sv_angle2, right_sv_angle2);
    new_blob_test.process_color_image(out, "sobel", "outline", 70);
//    new_blob_test.process_directory_of_color_ims("testing_car_stuff\\final_test\\in\\", "testing_car_stuff\\final_test\\out\\", 70);
//    new_blob_test.from_color_image(out, 70);
//    let mynewim = new_blob_test.create_outlined_image();
//    mynewim.save(out);

}

#[derive(Debug, Copy, Clone)]
pub struct VehicleBlock {
    left_x_ind: u32,
    top_y_ind: u32,
    width: u32,
    height: u32,
    lane: u8,
}

impl VehicleBlock {
    pub fn new(left_x_ind: u32, top_y_ind: u32, width: u32, height: u32, lane: u8) -> Self {
        VehicleBlock {
            left_x_ind,
            top_y_ind,
            width,
            height,
            lane, // will either be 0, 1, or 2 numbered from left to right
        }
    }
}

/// The structure used to perform simplified blob analysis on images from the first ~30 sec
/// from [this](https://www.youtube.com/watch?v=jjlBnrzSGjc) video. Technically, the library could
/// be used on any video that has 3 lanes of traffic and a similar overhead camera angle, but your mileage
/// may very as we had little luck finding other ones similar to this.
/// The inspiration for this idea came from
/// [this](https://pdfs.semanticscholar.org/d08b/b342d521f361d429e5aeed4012e87244b7cd.pdf)
/// 2015 paper by Ashwini B. and Yvaraju B N, though the actual blob analysis and most of the implementation details
/// used in this struct are much simpler than the technique they used.
pub struct ThreeLaneBlob {
    im: DynamicImage,
    left: SearchVector,
    middle: SearchVector,
    right: SearchVector,
    blocks: Vec<VehicleBlock>,
}

impl ThreeLaneBlob {

    /// # Examples
    ///
    /// You can create a new `ThreeLaneBlob` using a screenshot from the video like so:
    /// ```
    /// let my_im = image::open(filename_of_screenshot).unwrap();
    /// let left_sv = SearchVector::new((250, 50), (45, 615), 10);
    /// let mid_sv = SearchVector::new((300, 50), (300, 650), 10);
    /// let right_sv = SearchVector::new((350, 50), (570, 650), 10);
    /// let my_blob_test = ThreeLaneBlob::new(my_im, left_sv, mid_sv, right_sv);
    ///
    /// ```
    ///
    /// The `left` parameter is the area that will be searched in the left lane.
    /// The `middle` and `right` parameters denote the areas that will be searched in
    /// their respective lanes as well. For more info on how these work,
    /// see [SearchVector](struct.SearchVector.html).
    ///
    /// An important thing to note is that the `SearchVector` for the middle lane can handle having a an infinite slope (see `mid_sv` above),
    /// while the `SearchVector`s for the other lanes cannot. This is meant to ensure that the lanes are always centered
    /// correctly, if any of them must have a zero slope.
    pub fn new(im: DynamicImage, left: SearchVector, middle: SearchVector, right: SearchVector) -> Self {
        ThreeLaneBlob {
            im,
            left,
            middle,
            right,
            blocks: Vec::new(),
        }
    }


    /// Allows for the processing of multiple images in sequence (however many are in `input_directory`.
    ///
    /// # Examples
    /// Since a `ThreeLaneBlob` must be already instantiated for this method to be called, you just need to
    /// make one with the first image of the directory to be processed, then run this method on the `ThreeLaneBlob`.
    ///
    /// ```
    /// let new_blob_test = ThreeLaneBlob::new(.......);
    /// new_blob_test.process_directory_of_color_ims("test_in\\", "test_out\\", 70);
    /// ```
    pub fn process_directory_of_color_ims(&mut self, input_directory: &str, output_directory: &str, sobel_threshold: u8) {
        let paths = read_dir(input_directory).expect("issue opening directory.");

        let mut i = 0;
        for item in paths {
            let fname = item.expect("error accessing image").path().display().to_string();
            self.im = image::open(fname.as_str()).expect("error opening image");
            self.process_color_image(output_directory, ("sobel".to_owned() + &i.to_string()).as_str(), ("outlined".to_owned() + &i.to_string()).as_str(), sobel_threshold);
            i += 1;
            self.blocks = Vec::new();
        }
    }

    /// Used to process an image that has already had a sobel filter applied to it.
    pub fn process_sobeled_image(&mut self, output_directory: &str) {
        let outlined_im = self.create_outlined_image();
        outlined_im.save((output_directory.to_owned() + "outlined.jpg").as_str());
    }

    /// Used to process an image that has not yet had a sobel filter applied to it.
    pub fn process_color_image(&mut self, output_directory: &str, sobel_output_name: &str, outlined_output_name: &str, sobel_threshold: u8) {
        self.sobel_detector((output_directory.to_owned() + sobel_output_name + ".jpg").as_str(), sobel_threshold);
        self.im = image::open((output_directory.to_owned() + sobel_output_name + ".jpg").as_str()).unwrap();
        let outlined_im = self.create_outlined_image();
        outlined_im.save((output_directory.to_owned() + outlined_output_name + ".jpg").as_str());
    }

    /// Returns the number of cars/blobs that it has found in the image.
    pub fn get_car_count(&self) -> usize {
        self.blocks.len()
    }

    fn find_car(&mut self, lane_num: u8, slope: i32) {
        let (mut x_ind, mut y_ind, max_y, width) = match lane_num {
            0 => (self.left.start.0, self.left.start.1, self.left.end.1, self.left.width_over_two),
            1 => (self.middle.start.0, self.middle.start.1, self.middle.end.1, self.middle.width_over_two),
            2 => (self.right.start.0, self.right.start.1, self.right.end.1, self.right.width_over_two),
            x => panic!("invalid lane number"),
        };

        let offset_x_every = slope.abs() as u32;
        let direction_of_offset = if slope > 0 {1i32} else if slope < 0 {-1i32} else {0i32};

        // this will keep track of the difference between our current y_index
        // and the original y_index. This is used so we know when to offset x_ind
        // by 1 pixel.
        let mut y_ind_difference = 0u32;

        while y_ind < max_y {

            // search a wide window, not just a line
            for j in (x_ind - width)..(x_ind + width + 1) {

                let this_px = self.im.get_pixel(j, y_ind).channels()[0];

                // we have the first edge for a car
                if this_px == 255 {

                    let top_y_ind = y_ind.clone();
                    let h = self.find_height(&mut x_ind, &mut y_ind, offset_x_every, direction_of_offset, &mut y_ind_difference);

                    // now we know the height of this car
                    // need to basically do the same thing but with it's width
                    let mdpt_y = top_y_ind + (h / 2);

                    let w = self.find_width(x_ind, y_ind, h);

                    // only add it as a block if its area is large enough
                    // this helps remove false positives
                    if h * w > 400 {
                        self.blocks.push(VehicleBlock::new(x_ind, top_y_ind, w, h, lane_num));
                    }

                }

            }
            y_ind += 1;
            y_ind_difference += 1;

            let amount_of_offsets = y_ind_difference / offset_x_every;
            if amount_of_offsets > 0  {
                x_ind = (x_ind as i32 + (amount_of_offsets as i32 * direction_of_offset)) as u32;
                y_ind_difference = 0;
            }

        }

    }

    fn create_outlined_image(&mut self) -> image::RgbImage {
        let mut new_im = self.im.clone().to_rgb();
        let left_slope = self.left.slope;
        let mid_slope = if self.middle.slope == 0 {(self.im.dimensions().1 + 10) as i32} else {self.middle.slope};
        let right_slope = self.right.slope;

        self.find_car(0, left_slope);
        self.find_car(1, mid_slope);
        self.find_car(2, right_slope);

        println!("{:?}", self.blocks);

        for block in self.blocks.clone() {

            // figure out what x index we need to deal with
            // depending on which lane we're examining
            let x_ind = match block.lane {
                0 => self.left.start.0 - ((block.top_y_ind - self.left.start.1) / left_slope.abs() as u32),
                1 => {
                    // flag for zero slope middle lane
                    if mid_slope > self.im.dimensions().1 as i32 {
                        self.middle.start.0
                    }
                    else {
                        self.middle.start.0 - ((block.top_y_ind - self.middle.start.1) / mid_slope.abs() as u32)
                    }
                },
                2 => self.right.start.0 + ((block.top_y_ind - self.right.start.1) / right_slope.abs() as u32),
                _x => panic!("unexpected lane"), // ok to panic here... we know the only lanes being input will be 0, 1, or 2.
            };

            let this_top_y = block.top_y_ind;
            let half_width = block.width / 2;

            let mut green_px = new_im.get_pixel(0,0).clone();
            green_px.channels_mut()[1] = 100;

            let start = if x_ind > half_width {x_ind - half_width} else {0};
            // top and bottom lines
            // "from left side of block to right side of block"
            for x in start..(x_ind + (half_width)) {
                // top
                new_im.put_pixel(x, this_top_y, green_px);
                new_im.put_pixel(x, this_top_y + 1, green_px);

                // bottom
                new_im.put_pixel(x, this_top_y + block.height, green_px);
                new_im.put_pixel(x, this_top_y + block.height + 1, green_px);
            }

            // side lines
            // "from top of block to bottom of block"
            for y in this_top_y..(this_top_y + block.height) {
                // left side
                new_im.put_pixel(start, y, green_px);
                new_im.put_pixel(start + 1, y, green_px);

                //right side
                new_im.put_pixel(x_ind + half_width, y, green_px);
                new_im.put_pixel(x_ind + half_width + 1, y, green_px);

            }



        }

        new_im


    }

    fn find_height(&self, x_ind: &mut u32, y_ind: &mut u32, offset_x_every: u32, direction_of_offset: i32, y_ind_difference: &mut u32) -> u32{
        let top_ind_y = y_ind.clone(); // the top of the car
        let mut bot_ind_y = y_ind.clone(); // initialized as top of car, will become bottom
        let mut car_here = true; // there is a car here

//        println!("top_ind_y = {}", top_ind_y);

        while car_here {
            car_here = false;

            // be sure we're not going to go off the bottom of the image
            if bot_ind_y > self.im.dimensions().1 - 31 {
                bot_ind_y = self.im.dimensions().1 - 2;
                break;
            }

            // now search a 20x30 window for a white pixel
            // if there is a white pixel, we set it as our bottom y index and continue
            for window_y in bot_ind_y + 1..(bot_ind_y + 31) {
                for window_x in (*x_ind - 10)..(*x_ind + 11) {

                    // if this pixel is white, we're still somewhere in the middle of the car
                    // we then want to restart the window searching down from this pixel
                    if self.im.get_pixel(window_x, window_y).channels()[0] > 0 {
                        car_here = true;
                        bot_ind_y = window_y;
                        *y_ind_difference += bot_ind_y - *y_ind;
                        *y_ind = bot_ind_y;

                        // figure out how far we need to move x to stay in our search vector
                        let amount_of_offsets = *y_ind_difference / offset_x_every;
                        if amount_of_offsets > 0  {
                            *x_ind = (*x_ind as i32 + (amount_of_offsets as i32 * direction_of_offset)) as u32;
                            *y_ind_difference = 0;
                        }

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

    fn find_width(&self, x_ind: u32, y_ind: u32, height: u32) -> u32{

        // begin by sweeping left with a search window
        // from the point that we know to be the middle height
        // point of the car
        let mut leftmost_x_ind = x_ind.clone();
        let mut car_here=  true;
        while car_here {
            car_here = false;


            // create 20 x 20 window,
            // sweep along x first
            for window_y in (y_ind - ((height / 2) / 3))..(y_ind + ((height / 2) / 3)) {
                if leftmost_x_ind > 21 {
                    for window_x in (leftmost_x_ind - 21)..(leftmost_x_ind - 1) {
                        if self.im.get_pixel(window_x, window_y).channels()[0] > 0 {
                            car_here = true;
                            leftmost_x_ind = window_x;
                            break;
                        }
                    }
                    if car_here {break;}
                }
                else {
                    leftmost_x_ind = 0;
                    break;
                }
            }
        }

        (x_ind - leftmost_x_ind) * 2
    }

    fn sobel_detector(&self, export_path: &str, threshold: u8) {


        let (dim_x, dim_y) = self.im.dimensions();

        let gray_im = self.im.grayscale();

        let filt_x: &[f32] = &[-1f32,0f32,1f32,-2f32,0f32,2f32,-1f32,0f32,1f32];
        let filt_y: &[f32] = &[-1f32,-2f32, -1f32, 0f32,0f32, 0f32, 1f32, 2f32,1f32];
        let gray_x = gray_im.filter3x3(filt_x);
        let gray_y = gray_im.filter3x3(filt_y);

        let mut sobeled_im = image::ImageBuffer::new(dim_x, dim_y);


        for i in 0..dim_x {
            for j in 0..dim_y {
                let mut px1_sq = gray_x.get_pixel(i, j).channels()[0] as f32;
                let mut px2_sq = gray_y.get_pixel(i, j).channels()[0] as f32;

                px1_sq = px1_sq.powf(2.);
                px2_sq = px2_sq.powf(2.);

                let new_val = (px1_sq + px2_sq).sqrt().ceil() as u8;
                let new_px = gray_x.get_pixel(i, j).map(|_v| if new_val > threshold {255} else {0});
                sobeled_im.put_pixel(i, j, new_px);
            }
        }

        sobeled_im.save(export_path);
    }



}

/// Used to keep track of the area of an image that will be searched.
///
/// For the purposes of this project, that will be a highway lane.
///
/// # Examples
///
/// To search a box of width 20 pixels that runs from some pixel (250, 50) to (45, 615),
/// we could create a search vector like so:
///
/// ```
/// let left_sv = let left_sv = SearchVector::new((250, 50), (45, 615), 10);
/// ```
///
/// The SearchVector will internally keep track of slope to ensure that the area is searched
/// for blobs correctly, meaning that SearchVectors can be angled. It is important when using this struct in conjuntion
/// with `ThreeLaneBlob` that they
/// have a slope with absolute value > 1. The higher the slopes absolute value, the better the results will be
/// due to the underlying implementation of that struct.
pub struct SearchVector {
    start: (u32, u32),
    end: (u32, u32),
    width_over_two: u32,
    slope: i32,
}

impl SearchVector {
    pub fn new(start: (u32, u32), end: (u32, u32), width_over_two: u32) -> Self {
        SearchVector {
            start,
            end,
            width_over_two,
            slope:
            {
                if end.0 as f32 - start.0 as f32 == 0. {0i32}
                else {
                    let val = ((end.1 as f32 - start.1 as f32) / (end.0 as f32 - start.0 as f32)) as f32;
                    if val < 0. {val.floor() as i32} else {val.ceil() as i32}
                }
            },
        }
    }
}

#[cfg(test)]
mod SearchVectorTests {
    use super::SearchVector;

    #[test]
    fn check_best_left_slope() {
        let left_sv = SearchVector::new((250, 50), (45, 615), 10);
        assert_eq!(left_sv.slope, -3.);
    }

    #[test]
    fn check_best_mid_slope() {
        let mid_sv = SearchVector::new((300, 50), (300, 650), 10);
        assert_eq!(mid_sv.slope, 0.);
    }

    #[test]
    fn check_best_right_slope() {
        let right_sv = SearchVector::new((350, 50), (570, 650), 10);
        assert_eq!(right_sv.slope, 3.);
    }

}



extern crate image;
use image::{GenericImage, Pixel, DynamicImage};

pub fn count_example() {
    let img = image::open("out_images\\sobel.jpg").unwrap();

    let new_blob_test = Three_Lane_Blob::new(img, (0,0), (323, 63), (0,0));
    new_blob_test.find_car_middle();

}

pub fn count_cars(filename: &str, areas_of_interest: &Vec<vehicle_block>) {

}

#[derive(Debug)]
pub struct vehicle_block {
    top_y_ind: u32,
    width: u32,
    height: u32,
}


pub struct Three_Lane_Blob {
    im: DynamicImage,
    left: (u32, u32),
    middle: (u32, u32),
    right: (u32, u32),
}

pub struct Search_Vector {
    start: (u32, u32),
    end: (u32, u32),
    width: u32,
}


impl Three_Lane_Blob {
    pub fn new(im: DynamicImage, left: (u32, u32), middle: (u32, u32), right: (u32, u32)) -> Self {
        Three_Lane_Blob {
            im,
            left,
            middle,
            right,
        }
    }

    pub fn find_car_middle(&self) {
        let mut x_ind = self.middle.0;
        let mut y_ind = self.middle.1;

        let mut i = y_ind.clone();
        let mut payload = vehicle_block::new(0,0,0);



        while y_ind < 300 {

            // search a wide window, not just a line
            for j in (x_ind - 10)..(x_ind + 11) {
                let this_px = self.im.get_pixel(j, y_ind).channels()[0];


                // we have the first edge for a car
                if this_px > 0 {

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
//                                println!("checking pixel: {}, {}", window_x, window_y);
                                if self.im.get_pixel(window_x, window_y).channels()[0] > 0 {
                                    car_here = true;
                                    bot_ind_y = window_y;
                                    y_ind = bot_ind_y;
//                                    println!("bot_ind_y is now {}", bot_ind_y);
                                    break;
                                }
                            }
                            if car_here == true {
//                                println!("found a car at y = {}, breaking this loop", window_y);
                                break;
                            }
                        }
                    }

                    payload = vehicle_block::new(top_ind_y, 0, bot_ind_y - top_ind_y);
                }

            }
            y_ind += 1;
        }



        println!("{:?}", payload);
    }
}

impl vehicle_block {
    pub fn new(top_y_ind: u32, width: u32, height: u32) -> Self {
        vehicle_block {
            top_y_ind,
            width,
            height,
        }
    }
}


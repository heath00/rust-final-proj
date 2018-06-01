extern crate rust_im;
use rust_im::sobel::{sobel_detector};
use rust_im::eigenfaces::{handler};

fn main() {
    sobel_detector("in_images\\test2.jpg",
                   "out_images",
                   40);

//    handler()



}

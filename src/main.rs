extern crate rust_im;
use rust_im::sobel::{sobel_detector};
use rust_im::traffic_detection::{count_example};

fn main() {
    count_example("testing_car_stuff\\final_test_angle_2\\1.png", "testing_car_stuff\\final_test_angle_2\\");

    //sobel_detector("in_images\\test2.jpg",
    //              "out_images",
    //             40);
    let vin = vec![60, 2];
    let vout = vec![2, 60];
    encrypt_many("in_images\\test.jpg",
                 "out_images",vin);

    decrypt_many("out_images\\encrImg.jpg",
                 "out_images", vout);


//    handler()

//    sobel_detector("three_lane\\left_and_middle1.png",
//                   "three_lane\\sobel_left_and_middle1.jpg",
//                   70);
//
//    count_example("three_lane\\sobel_left_and_middle1.jpg", "three_lane\\lined1.jpg");
//
//    sobel_detector("three_lane\\left_and_middle2.png",
//                   "three_lane\\sobel_left_and_middle2.jpg",
//                   70);
//
//    count_example("three_lane\\sobel_left_and_middle2.jpg", "three_lane\\lined2.jpg")


//    handler()



}

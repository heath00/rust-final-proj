extern crate rust_im;
use rust_im::sobel::{sobel_detector};
use rust_im::imgEncrypt::{encrypt,decrypt,encrypt_many,decrypt_many};
use rust_im::traffic_detection::{count_example, class_demo};

fn main() {
//    count_example("testing_car_stuff\\final_test_angle_2\\1.png", "testing_car_stuff\\final_test_angle_2\\");
    class_demo();
    sobel_detector("in_images\\test.jpg",
                  "out_images",
                 40);
    let vin = vec![60, 2];
    let vout = vec![2, 60];
    encrypt_many("in_images\\test.jpg",
                 "out_images",vin);

    decrypt_many("out_images\\encrImg.jpg",
                 "out_images", vout);


//    handler()



}

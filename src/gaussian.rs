pub fn gaussian_blur_filter<T: Pixel>(){
    let img = image::open(filename).unwrap();
    let gray_im = img.grayscale();
}

pub fn compute_guassian_kernel2d(width: usize, sigma: f32) -> Vec<f32> {

        let half_width = (width + 1) / 2;
//      let sigma: f32 = match sigma {
//          0f32...0.00001f32 => 0.3 * ((w as f32 - 1.0) * 0.5 - 1.0) + 0.8,
//      }

        let sigma2: f32 = 2.0 / sigma * sigma;
        let kernel_vector = vec![0f32; width];

        // create filter
        for x in 0..half_width{
            let t = (half_width - x) as f32;
            let tt: f32 = (-t * t / sigma2).exp_m1();
            kernel_vector[x] = tt + 1f32;
        }

        for y in half_width..width{
            k[y] = k[width - y - 1];
        }

    let mut sum = 0.0;

    // Normalize
    for i in 0..width { sum += k[i]; }
    for i in 0..width { k[i] /= sum; }

    // Return Gaussian Kernel
    kernel_vector

}
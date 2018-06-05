pub fn compute_gaussian_kernel2d(width: usize, sigma: f32) -> Vec<f32> {

        let half_width = (width + 1) / 2;

        let sigma2: f32 = 2.0 / sigma * sigma;
        let mut kernel_vector = vec![0f32; width];

        // create filter
        for x in 0..half_width{
            let t = (half_width - x) as f32;
            let tt: f32 = (-t * t / sigma2).exp_m1();
            kernel_vector[x] = tt + 1f32;
        }

        for y in half_width..width{
            kernel_vector[y] = kernel_vector[width - y - 1];
        }

    let mut sum = 0.0;

    // Normalize
    for i in 0..width { sum += kernel_vector[i]; }
    for i in 0..width { kernel_vector[i] /= sum; }

    // Return Gaussian Kernel
    kernel_vector

}

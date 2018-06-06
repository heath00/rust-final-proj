/// A function to compute a gaussian kernel of size: s with standard deviation: sd
pub fn compute_gaussian_kernel2d(size: usize, sd: f32) -> Vec<f32> {


        let half_size = (size + 1) / 2;

        let sd_adjusted: f32 = 2.0 / sd * sd;
        let mut kernel_vector = vec![0 as f32; size];

        // create filter
        for x in 0..half_size{
            let t = (half_size - x) as f32;
            let tt: f32 = (-t * t / sd_adjusted).exp_m1();
            kernel_vector[x] = tt + 1 as f32;
        }

        for y in half_size..size{
            kernel_vector[y] = kernel_vector[size - y - 1];
        }

    let mut sum = 0.0;

    // Normalize
    for i in 0..size { sum += kernel_vector[i]; }
    for i in 0..size { kernel_vector[i] /= sum; }

    // Return Gaussian Kernel
    kernel_vector

}

fn main() 
{
    // Power of 2
    let fft = platform_aware_rs::FFT
        {
            planner_size: 8,
            re: 1124.0f32,
            im: 2047.1f32
        };
    
    fft.fft_fowarding();

    println!();

    // Not power of 2
    let fft = platform_aware_rs::FFT
    {
        planner_size: 9,
        re: 1124.0f32,
        im: 2047.1f32
    };

    fft.fft_fowarding();
}

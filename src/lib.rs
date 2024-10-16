#![allow(unused_imports)]

use sysinfo::System;
use std::arch::x86_64::*;

pub struct FFT
{
    pub planner_size: usize,
    pub re: f32,
    pub im: f32,
}

impl FFT 
{
    pub fn fft_fowarding(&self)
    {
        if (self.planner_size & (self.planner_size - 1)) != 0
        {

            use rustfft::{FftPlanner, num_complex::Complex};

            let mut planner = FftPlanner::new();
            let fft = planner.plan_fft_forward(self.planner_size);

            let mut buffer = vec![Complex{ re: self.re, im: self.im }; self.planner_size];

            println!("buffer before FFT: {:?}\n\n\n", buffer);

            fft.process(&mut buffer);

            println!("buffer affter FFT: {:?}", buffer);
        }

        else  
        {
            use rustfft::{Fft, FftDirection, num_complex::Complex, algorithm::Radix4};

            let fft = Radix4::new(self.planner_size, FftDirection::Forward);

            let mut buffer = vec![Complex{ re: self.re, im: self.im }; self.planner_size];
            println!("buffer before FFT: {:?}\n\n\n", buffer);

            fft.process(&mut buffer);

            println!("buffer affter FFT: {:?}", buffer);
        }
    }
}

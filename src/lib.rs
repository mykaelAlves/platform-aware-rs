use sysinfo::System;
use std::arch::x86_64::*;

//
// testing
//

pub fn get_packed_32bit() -> __m256i
{
    unsafe 
    {
        _mm256_set_epi32(1, 2, 3, 4, 5, 6, 7, 8)
    }
}
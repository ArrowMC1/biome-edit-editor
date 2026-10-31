unsafe extern "C" {
    fn c_get_biome_colour_square_parallel(
        seed: i64,
        start_x: i32,
        start_z: i32,
        length: i32,
        height: i32,
        num_threads: i32,
        thread_no: i32,
        results: *mut u32,
    );
}


pub fn get_biome_colour_square_parallel(seed: i64,
        start_x: i32,
        start_z: i32,
        length: i32,
        height: i32,
        num_threads: i32,
        thread_no: i32,
        results: *mut u32,)
    {
    unsafe { c_get_biome_colour_square_parallel(seed, start_x, start_z, length, height, num_threads, thread_no, results) }
}
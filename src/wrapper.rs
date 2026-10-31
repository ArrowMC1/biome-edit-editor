use rayon::prelude::*;

unsafe extern "C" {
    fn c_get_biome_colour_square_parallel(
        seed: i64,
        start_x: i32,
        start_z: i32,
        length: i32,
        height: i32,
        num_threads: i32,
        thread_no: i32
    );

    fn c_set_global_shift_noise(val: bool);

    fn c_set_global_main_map(ptr: *mut u32);

    fn c_set_global_noise_maps(
        _p_t: *mut u16,
        _p_h: *mut u16,
        _p_c: *mut u16,
        _p_e: *mut u16,
        _p_w: *mut u16
    );
}

fn get_biome_colour_square_parallel(seed: i64,
        start_x: i32,
        start_z: i32,
        length: i32,
        height: i32,
        num_threads: i32,
        thread_no: i32)
    {
    unsafe { c_get_biome_colour_square_parallel(seed, start_x, start_z, length, height, num_threads, thread_no) }
}

/// WARNING: Only Call This When No Other C ABI calls are running 
pub fn set_global_shift_noise(val: bool){
    unsafe {c_set_global_shift_noise(val)}
}

/// WARNING: Only Call This When No Other C ABI calls are running 
pub fn set_global_main_map(ptr: *mut u32){
    unsafe {c_set_global_main_map(ptr)}
}

/// WARNING: Only Call This When No Other C ABI calls are running
pub fn set_global_noise_maps(
        p_t: *mut u16,
        p_h: *mut u16,
        p_c: *mut u16,
        p_e: *mut u16,
        p_w: *mut u16){
    unsafe {c_set_global_noise_maps(p_t, p_h, p_c, p_e, p_w)}
}


/// This Uses the the maps supplied by set_global_noise_maps and set_global_main_map.
/// if these are not set beforehand, this will cause a null pointer derefrence
pub fn gen_biomes_multithreaded(width: i32, height: i32, num_threads: i32) {

    assert!(width > 0, "Width must be positive");
    assert!(height > 0, "Height must be positive");
    assert!(num_threads > 0, "Thread count must be positive");

    (0..num_threads).into_par_iter().for_each(|thread_no| {
        get_biome_colour_square_parallel(
            0,
            -(width/2),
            -(height/2),
            width,
            height,
            num_threads,
            thread_no
        );
    });
}
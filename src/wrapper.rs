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

    fn c_set_global_write_mode(val: bool);

    fn c_set_global_main_map(ptr: *mut u32);

    fn c_set_global_height_map(ptr: *mut u8);

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

/// WARNING: Only Call This When No Other C ABI calls are running!
/// Decides whether to use local distortion in the biome map.
/// Setting this to true creates a more accurate biome map but not all 
/// noise points will be filled when using write mode
pub fn set_global_shift_noise(val: bool){
    unsafe {c_set_global_shift_noise(val)}
}

/// WARNING: Only Call This When No Other C ABI calls are running!
/// Turning write mode on generates the noise values and writes them
/// to pointers provided by set_global_noise_maps. otherwise it will read from it.
/// WARNING: if set_global_noise_maps is not set beforehand, this will cause a null pointer derefrence!
pub fn set_global_write_mode(val: bool){
    unsafe {c_set_global_write_mode(val)}
}

/// WARNING: Only Call This When No Other C ABI calls are running!
fn set_global_main_map(ptr: *mut u32){
    unsafe {c_set_global_main_map(ptr)}
}

/// WARNING: Only Call This When No Other C ABI calls are running!
fn set_global_height_map(ptr: *mut u8){
    unsafe {c_set_global_height_map(ptr)}
}


/// WARNING: Only Call This When No Other C ABI calls are running!
fn set_global_noise_maps(
        p_t: *mut u16,
        p_h: *mut u16,
        p_c: *mut u16,
        p_e: *mut u16,
        p_w: *mut u16){
    unsafe {c_set_global_noise_maps(p_t, p_h, p_c, p_e, p_w)}
}


pub struct MapGenerator{
    pub width: i32,
    pub height: i32,
    pub main_map: Vec<u32>,
    pub height_map: Vec<u8>,
    pub t_map: Vec<u16>,
    pub h_map: Vec<u16>,
    pub c_map: Vec<u16>,
    pub e_map: Vec<u16>,
    pub w_map: Vec<u16>,

}

impl MapGenerator{
    pub fn new(width: i32, height: i32) -> Self{
        let size: usize = (width * height) as usize;

        Self { 
            width: width,
            height: height,
            main_map: vec![0x000000FF; size],
            height_map: vec![0x00; size],
            t_map: vec![0x0000; size],
            h_map: vec![0x0000; size],
            c_map: vec![0x0000; size],
            e_map: vec![0x0000; size],
            w_map: vec![0x0000; size],

         }
    }


    pub fn reduce_continental_circle(
        &mut self,
        center_x: usize,
        center_z: usize,
        radius: f32,
        strength: f32,
        falloff: f32,
    ) {
        let width = self.width as usize;
        let height = self.height as usize;

        if width == 0 || height == 0 || radius <= 0.0 {
            return;
        }

        let min_x = (center_x as f32 - radius).max(0.0) as usize;
        let max_x = ((center_x as f32 + radius) as usize).min(width - 1);

        let min_z = (center_z as f32 - radius).max(0.0) as usize;
        let max_z = ((center_z as f32 + radius) as usize).min(height - 1);

        let radius_squared = radius * radius;

        for z in min_z..=max_z {
            for x in min_x..=max_x {

                // Bounds checking
                let index = z * width + x;

                if index >= self.c_map.len() {
                    continue;
                }

                let dx = x as f32 - center_x as f32;
                let dz = z as f32 - center_z as f32;

                let distance_squared = dx * dx + dz * dz;

                if distance_squared > radius_squared {
                    continue;
                }

                let normalized_distance = distance_squared.sqrt() / radius;

                let effect = (1.0 - normalized_distance).powf(falloff);

                let reduction = (strength.clamp(0.0, 1.0)
                    * effect
                    * u16::MAX as f32) as u16;

                self.e_map[index] = self.e_map[index].saturating_sub(reduction);
            }
        }
    }

    /// This Uses the the maps supplied by set_global_noise_maps and set_global_main_map.
    /// WARNING: if these are not set beforehand, this will cause a null pointer derefrence!
    pub fn gen_biomes_multithreaded(&mut self, num_threads: i32) {

        assert!(self.width > 0, "Width must be positive");
        assert!(self.height > 0, "Height must be positive");
        assert!(num_threads > 0, "Thread count must be positive");

        set_global_height_map(self.height_map.as_mut_ptr());
        set_global_main_map(self.main_map.as_mut_ptr());
        set_global_noise_maps(
            self.t_map.as_mut_ptr(), 
            self.h_map.as_mut_ptr(), 
            self.c_map.as_mut_ptr(), 
            self.e_map.as_mut_ptr(), 
            self.w_map.as_mut_ptr()
        );

        (0..num_threads).into_par_iter().for_each(|thread_no| {
            get_biome_colour_square_parallel(
                5674997078289778296,
                -(self.width/2),
                -(self.height/2),
                self.width,
                self.height,
                num_threads,
                thread_no
            );
        });
    }
}
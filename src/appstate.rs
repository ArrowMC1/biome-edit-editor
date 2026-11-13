use crate::enums::{MainMap, NoiseMap};
use crate::image::{Image, Images};
use crate::image_util::create_shaded_relief_map;
use crate::wrapper::{MapGenerator, set_global_shift_noise, set_global_write_mode};

pub struct AppState{
    pub images: Images,
    pub map_generator: MapGenerator,
    pub hover_pos: Option<(usize, usize)>,
    pub active_map: MainMap,
    pub active_noise_map: NoiseMap
}

impl Default for AppState {
    fn default() -> Self {

        let images: Images = Images::new(800, 800, 300, 300, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        let map_generator = MapGenerator::new(crate::IMG_SIZE, crate::IMG_SIZE);
        let hover_pos = None;
        let active_map = MainMap::Main;
        let active_noise_map = NoiseMap::Continental;

        Self {images, map_generator, hover_pos, active_map, active_noise_map}
    }
}

impl AppState{


    /// Returns a refrence to the active main map,
    /// a mutable one to the image (main maps dont have data)
    pub fn active_map_mut(&mut self) -> &mut Image {
        match self.active_map {
            MainMap::Main         => &mut self.images.main_map,
            MainMap::ShadedRelief => &mut self.images.shaded_relief_map,
            MainMap::Heightmap    => &mut self.images.height_map,
        }
    }

    /// Returns a refrence to the active noise map,
    /// a mutable one to the data
    pub fn active_noisemap_img_mut(&mut self) -> &mut Image {
        match self.active_noise_map {
            NoiseMap::Temperature => &mut self.images.t_map,
            NoiseMap::Humidity    => &mut self.images.h_map,
            NoiseMap::Continental => &mut self.images.c_map,
            NoiseMap::Erosion     => &mut self.images.e_map,
            NoiseMap::Weirdness   => &mut self.images.w_map,
        }
    }

    /// Returns a refrence to the active noise map,
    /// a mutable one to the data
    pub fn active_noisemap_data_mut(&mut self) -> &mut Vec<u16> {
        match self.active_noise_map {
            NoiseMap::Temperature => &mut self.map_generator.t_map,
            NoiseMap::Humidity    => &mut self.map_generator.h_map,
            NoiseMap::Continental => &mut self.map_generator.c_map,
            NoiseMap::Erosion     => &mut self.map_generator.e_map,
            NoiseMap::Weirdness   => &mut self.map_generator.w_map,
        }
    }


    /// Returns 2 refrences to the active noise map,
    /// a mutable one to the image, and an immutabe one to the data
    pub fn active_noisemap_dual_refs(&mut self) -> (&mut Image, &Vec<u16>) {
        match self.active_noise_map {
            NoiseMap::Temperature => (&mut self.images.t_map, &self.map_generator.t_map),
            NoiseMap::Humidity    => (&mut self.images.h_map, &self.map_generator.h_map),
            NoiseMap::Continental => (&mut self.images.c_map, &self.map_generator.c_map),
            NoiseMap::Erosion     => (&mut self.images.e_map, &self.map_generator.e_map),
            NoiseMap::Weirdness   => (&mut self.images.w_map, &self.map_generator.w_map),
        }
    }

    /*pub fn active_noisemap_data_ref(&self) -> &Vec<u16> {
        match self.active_noise_map {
            ActiveNoiseMap::Temperature => &self.map_generator.t_map,
            ActiveNoiseMap::Humidity    => &self.map_generator.h_map,
            ActiveNoiseMap::Continental => &self.map_generator.c_map,
            ActiveNoiseMap::Erosion     => &self.map_generator.e_map,
            ActiveNoiseMap::Weirdness   => &self.map_generator.w_map,
        }
    }*/

    //Really inefficient, generates whole biome map and heightmap twice TODO: FIX
    pub fn initialise_maps(&mut self){
        set_global_write_mode(true);
        set_global_shift_noise(false);
        self.map_generator.gen_biomes_multithreaded(32);
        set_global_shift_noise(true);
        set_global_write_mode(false);

        self.map_generator.gen_biomes_multithreaded(32);

        
        self.images.main_map.copy_u32_color_data(&self.map_generator.main_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        self.images.height_map.copy_u8_greyscale_data(&self.map_generator.height_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        self.images.t_map.copy_u16_greyscale_data(&self.map_generator.t_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        self.images.h_map.copy_u16_greyscale_data(&self.map_generator.h_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        self.images.c_map.copy_u16_greyscale_data(&self.map_generator.c_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        self.images.e_map.copy_u16_greyscale_data(&self.map_generator.e_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);
        self.images.w_map.copy_u16_greyscale_data(&self.map_generator.w_map, 0, 0, crate::IMG_SIZE as usize, crate::IMG_SIZE as usize);

        create_shaded_relief_map(
            &mut self.images.shaded_relief_map, 
            &self.map_generator.main_map, 
            &self.map_generator.height_map, 
            crate::IMG_SIZE, crate::IMG_SIZE);
        
        self.images.shaded_relief_map.mark_all_dirty();
    }


    pub fn update_maps(&mut self){
        self.map_generator.gen_biomes_multithreaded(32);

        
        self.images.main_map.copy_u32_color_data(&self.map_generator.main_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);
        self.images.height_map.copy_u8_greyscale_data(&self.map_generator.height_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);
        self.images.t_map.copy_u16_greyscale_data(&self.map_generator.t_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);
        self.images.h_map.copy_u16_greyscale_data(&self.map_generator.h_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);
        self.images.c_map.copy_u16_greyscale_data(&self.map_generator.c_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);
        self.images.e_map.copy_u16_greyscale_data(&self.map_generator.e_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);
        self.images.w_map.copy_u16_greyscale_data(&self.map_generator.w_map, 0, 0, crate::IMG_SIZE, crate::IMG_SIZE);

        create_shaded_relief_map(
            &mut self.images.shaded_relief_map, 
            &self.map_generator.main_map, 
            &self.map_generator.height_map, 
            crate::IMG_SIZE, crate::IMG_SIZE);
        
        self.images.shaded_relief_map.mark_all_dirty();
    }
}
use egui::{Color32, TextureHandle, TextureOptions};

pub struct Image {
    visual_width: usize,
    visual_height: usize,
    logical_width: usize,
    logical_height: usize,
    pixels: Vec<u32>,
    texture: Option<TextureHandle>,
    dirty: bool,
}

impl Image {
    pub fn new(visual_width: usize, visual_height: usize, logical_width: usize, logical_height: usize) -> Self {
        Self {
            visual_width,
            visual_height,
            logical_width,
            logical_height,
            pixels: vec![0x000000FF; logical_width * logical_height],
            texture: None,
            dirty: false,
        }
    }

    fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.logical_width || y >= self.logical_height {
            panic!("X or Y Above Logical Width/Height");
            //return;
        }

        self.pixels[y * self.logical_width + x] = color;
    }

    fn send_texture_to_gpu_internal(&mut self, ui: &mut egui::Ui) {
        // Convert our u32 RGBA pixels into egui's ColorImage.
        let pixels: Vec<Color32> = self
            .pixels
            .iter()
            .map(|&pixel| {
                let r = ((pixel >> 24) & 0xFF) as u8;
                let g = ((pixel >> 16) & 0xFF) as u8;
                let b = ((pixel >> 8) & 0xFF) as u8;
                let a = (pixel & 0xFF) as u8;

                Color32::from_rgba_unmultiplied(r, g, b, a)
            })
            .collect();

            let image = egui::ColorImage {
                size: [self.logical_width, self.logical_height],
                source_size: egui::vec2(
                    self.logical_width as f32,
                    self.logical_height as f32,
                ),
                pixels,
            };

        match &mut self.texture {
            Some(texture) => {
                texture.set(image, TextureOptions::NEAREST);
            }
            None => {
                self.texture = Some(ui.ctx().load_texture(
                    "main-map",
                    image,
                    TextureOptions::NEAREST,
                ));
            }
        }
    }

    pub fn send_texture_to_gpu(&mut self){
        self.dirty = true;
    }


    pub fn show_texture(&mut self, ui: &mut egui::Ui) {

        if self.dirty{
            self.dirty = false;
            self.send_texture_to_gpu_internal(ui);
        }

        if let Some(texture) = &self.texture {
            ui.image((
                texture.id(),
                egui::vec2(self.visual_width as f32, self.visual_height as f32),
            ));
        }
    }
}

use rayon::prelude::*;
use crate::wrapper::get_biome_colour_square_parallel;

struct SharedOutput(*mut u32);

// SAFETY: Each Rayon task writes to a disjoint set of indices.
unsafe impl Send for SharedOutput {}
unsafe impl Sync for SharedOutput {}

impl SharedOutput {
    fn as_ptr(&self) -> *mut u32 {
        self.0
    }
}

impl Image {
    pub fn gen_biomes_multithreaded(&mut self) {
        let num_threads = 32;

        // Copy dimensions before sharing the pointer.c
        let width = self.logical_width as i32;
        let height = self.logical_height as i32;

        let output = SharedOutput(self.pixels.as_mut_ptr());

        (0..num_threads).into_par_iter().for_each(|thread_no| {
            get_biome_colour_square_parallel(
                0,
                -(width/2),
                -(height/2),
                width,
                height,
                num_threads,
                thread_no,
                output.as_ptr(),
            );
        });
    }
}
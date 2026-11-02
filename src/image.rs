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

    pub fn copy_u32_color_data(&mut self, img_data: &Vec<u32>){
        self.pixels = img_data.clone();
        self.send_texture_to_gpu();
    }

    pub fn copy_u16_greyscale_data(&mut self, img_data: &Vec<u16>){

    for (dst, &src) in self.pixels.iter_mut().zip(img_data) {
        let gray = (src >> 8) as u32;

        *dst = (gray << 24)
            | (gray << 16)
            | (gray << 8)
            | 0xFF;
    }

        self.send_texture_to_gpu();
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


pub struct Images{
    pub main_map: Image,

    pub t_map: Image,
    pub h_map: Image,
    pub c_map: Image,
    pub e_map: Image,
    pub w_map: Image,
}

impl Images {
    pub fn new(
        visual_width_main: usize,
        visual_height_main: usize,
        visual_width_noise: usize,
        visual_height_noise: usize,
        logical_width: usize,
        logical_height: usize,
    ) -> Self {
        Self {
            main_map: Image::new(
                visual_width_main,
                visual_height_main,
                logical_width,
                logical_height,
            ),

            t_map: Image::new(
                visual_width_noise,
                visual_height_noise,
                logical_width,
                logical_height,
            ),

            h_map: Image::new(
                visual_width_noise,
                visual_height_noise,
                logical_width,
                logical_height,
            ),

            c_map: Image::new(
                visual_width_noise,
                visual_height_noise,
                logical_width,
                logical_height,
            ),

            e_map: Image::new(
                visual_width_noise,
                visual_height_noise,
                logical_width,
                logical_height,
            ),

            w_map: Image::new(
                visual_width_noise,
                visual_height_noise,
                logical_width,
                logical_height,
            ),
        }
    }
}
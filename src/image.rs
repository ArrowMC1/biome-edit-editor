use egui::{Color32, TextureHandle};

pub struct Image {
    visual_width: usize,
    visual_height: usize,
    pub logical_width: usize,
    pub logical_height: usize,
    pixels: Vec<u32>,
    texture: Option<TextureHandle>,
    dirty: bool,
    pub hover_pos: Option<(f32, f32)>,
    pub mouse_pos_uv: Option<(f32, f32)>,
    pub zoom_data: (f32, f32, f32)
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
            hover_pos: None,
            mouse_pos_uv: None,
            zoom_data: (1.0, 0.5, 0.5),
        }
    }


    fn clear_border(&mut self, thickness: usize) {
        let width = self.logical_width;
        let height = self.logical_height;

        for y in 0..height {
            for x in 0..width {
                if x < thickness
                    || x >= width.saturating_sub(thickness)
                    || y < thickness
                    || y >= height.saturating_sub(thickness)
                {
                    let index = y * width + x;
                    self.pixels[index] = 0x00000000;
                }
            }
        }
    }

    fn send_texture_to_gpu_internal(&mut self, ui: &mut egui::Ui) {
        self.clear_border(1);
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

        let texture_options = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            wrap_mode: egui::TextureWrapMode::ClampToEdge,
            mipmap_mode: None,
        };

        match &mut self.texture {
            Some(texture) => {
                texture.set(image, texture_options);
            }
            None => {
                self.texture = Some(ui.ctx().load_texture(
                    "main-map",
                    image,
                    texture_options,
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


    pub fn copy_u8_greyscale_data(&mut self, img_data: &Vec<u8>){

            for (dst, &src) in self.pixels.iter_mut().zip(img_data) {
                let gray = src as u32;

                *dst = (gray << 24)
                    | (gray << 16)
                    | (gray << 8)
                    | 0xFF;
            }

        self.send_texture_to_gpu();
    }



    pub fn show_texture(&mut self, ui: &mut egui::Ui) {
        if self.dirty {
            self.dirty = false;
            self.send_texture_to_gpu_internal(ui);
        }

        if let Some(texture) = &self.texture {
            let display_size = egui::vec2(
                self.visual_width as f32,
                self.visual_height as f32,
            );

            let uv_min = egui::pos2(
            (self.zoom_data.1 - (self.zoom_data.0 / 2.0)) as f32,
            (self.zoom_data.2 - (self.zoom_data.0 / 2.0)) as f32);

            let uv_max = egui::pos2(
            (self.zoom_data.1 + (self.zoom_data.0 / 2.0)) as f32,
            (self.zoom_data.2 + (self.zoom_data.0 / 2.0)) as f32);

            let uv = egui::Rect::from_min_max(
                uv_min,
                uv_max,
            );

            let response = ui.add(
                egui::Image::new((texture.id(), display_size))
                    .uv(uv),
            );

            if let Some(mouse_pos) = response.hover_pos() {
                let local = mouse_pos - response.rect.min;

                // Normalize mouse position within displayed image.
                let normalized_x = local.x / display_size.x;
                let normalized_y = local.y / display_size.y;

                // Map normalized coordinates into UV range.
                let uv_x = uv_min.x + normalized_x * (uv_max.x - uv_min.x);
                let uv_y = uv_min.y + normalized_y * (uv_max.y - uv_min.y);

                self.mouse_pos_uv = Some((uv_x, uv_y));

                // Convert UV coordinates into logical image coordinates.
                let x = uv_x * self.logical_width as f32;
                let y = uv_y * self.logical_height as f32;

                self.hover_pos = Some((x, y));

            } else {
                self.hover_pos = None;
            }
        }
    }

    pub fn handle_zooming(&mut self, scroll_y: f32){
        if let Some((_x, _z)) = self.hover_pos {

            const MIN_ZOOM: f32 = 0.001;
            const MAX_ZOOM: f32 = 2.0;
      
        
            if scroll_y != 0.0 {
                let old_zoom = self.zoom_data.0;
                let factor = 1.0 + scroll_y.abs() as f32 * 0.003;
                let new_zoom = if scroll_y < 0.0 {
                    old_zoom * factor
                } else {
                    old_zoom / factor
                };
                let t = 1.0 - new_zoom / old_zoom;
                if self.zoom_data.0 != MIN_ZOOM && self.zoom_data.0 != MAX_ZOOM{
                    self.zoom_data.1 += ((self.mouse_pos_uv.unwrap().0 - self.zoom_data.1) * t).clamp(-1.0, 1.0);
                    self.zoom_data.2 += ((self.mouse_pos_uv.unwrap().1 - self.zoom_data.2) * t).clamp(-1.0, 1.0);
                }
                self.zoom_data.1 = self.zoom_data.1.clamp(0.0, 1.0);
                self.zoom_data.2 = self.zoom_data.2.clamp(0.0, 1.0);
                self.zoom_data.0 = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
            }
        }
    }
}


pub struct Images{
    pub main_map: Image,
    pub height_map: Image,

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


            height_map: Image::new(
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

    pub fn set_zoom_data(&mut self, data: (f32, f32, f32) ){
        self.main_map.zoom_data = data;
        self.height_map.zoom_data = data;

        self.t_map.zoom_data = data;
        self.h_map.zoom_data = data;
        self.c_map.zoom_data = data;
        self.e_map.zoom_data = data;
        self.w_map.zoom_data = data;

    }
}
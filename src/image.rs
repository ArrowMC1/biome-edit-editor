use egui::{Color32, TextureHandle, Rect};


enum OverlayShape {
    Dot {
        x: usize,
        z: usize,
    },
    HollowCircle {
        x: usize,
        z: usize,
        radius: f32,
    },
    Pixel {
        x: usize,
        z: usize,
    },
}

pub struct Image {
    visual_width: usize,
    visual_height: usize,
    pub logical_width: usize,
    pub logical_height: usize,
    pub pixels: Vec<u32>,
    texture: Option<TextureHandle>,
    dirty: bool,
    dirty_rect: Option<(usize, usize, usize, usize)>,
    pub hover_pos: Option<(f32, f32)>,
    pub mouse_pos_uv: Option<(f32, f32)>,
    pub zoom_data: (f32, f32, f32),
    parent_window_rect: Rect,
    overlay_queue: Vec<OverlayShape>,
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
            dirty_rect: None,
            hover_pos: None,
            mouse_pos_uv: None,
            zoom_data: (1.0, 0.5, 0.5),
            parent_window_rect: egui::Rect::NOTHING,
            overlay_queue: Vec::new(),
        }
    }

    fn mark_dirty_region(&mut self, x: usize, z: usize, width: usize, height: usize) {
        if width == 0 || height == 0 {
            return;
        }

        let new_rect = (x, z, x + width, z + height);

        self.dirty_rect = Some(match self.dirty_rect {
            Some((old_x, old_z, old_x2, old_z2)) => (
                old_x.min(new_rect.0),
                old_z.min(new_rect.1),
                old_x2.max(new_rect.2),
                old_z2.max(new_rect.3),
            ),
            None => new_rect,
        });

        self.dirty = true;
    }

    pub fn mark_all_dirty(&mut self) {
        self.dirty_rect = Some((
            0,
            0,
            self.logical_width,
            self.logical_height,
        ));

        self.dirty = true;
    }

    fn send_texture_to_gpu_internal(&mut self, ui: &mut egui::Ui) {
        if !self.dirty && self.texture.is_some() {
            return;
        }

        let texture_options = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            wrap_mode: egui::TextureWrapMode::ClampToEdge,
            mipmap_mode: None,
        };

        // Initial texture creation always uploads the entire image.
        let (x, z, width, height) = if self.texture.is_none() || self.dirty_rect.is_none() {
            (
                0,
                0,
                self.logical_width,
                self.logical_height,
            )
        } else {
            let (x, z, x2, z2) = self.dirty_rect.unwrap();

            (x, z, x2 - x, z2 - z)
        };

        if width == 0 || height == 0 {
            return;
        }

        let mut pixels = Vec::with_capacity(width * height);

        for row in z..z + height {
            let row_start = row * self.logical_width;

            for col in x..x + width {
                let pixel = self.pixels[row_start + col];

                // Transparent outer border.
                let pixel = if col == 0
                    || row == 0
                    || col == self.logical_width - 1
                    || row == self.logical_height - 1
                {
                    0x00000000
                } else {
                    pixel
                };

                let r = (pixel >> 24) as u8;
                let g = (pixel >> 16) as u8;
                let b = (pixel >> 8) as u8;
                let a = pixel as u8;

                pixels.push(Color32::from_rgba_unmultiplied(r, g, b, a));
            }
        }

        let image = egui::ColorImage {
            size: [width, height],
            source_size: egui::vec2(width as f32, height as f32),
            pixels,
        };

        match &mut self.texture {
            Some(texture) => {
                texture.set_partial([x, z], image, texture_options);
            }
            None => {
                // First upload must use the full image dimensions.
                self.texture = Some(ui.ctx().load_texture(
                    "main-map",
                    image,
                    texture_options,
                ));
            }
        }

        self.dirty_rect = None;
        self.dirty = false;
    }

    fn get_copy_region(
        &self,
        data_len: usize,
        start_x: usize,
        start_z: usize,
        x_size: usize,
        z_size: usize,
    ) -> Option<(usize, usize)> {
        if crate::IMG_SIZE == 0 {
            return None;
        }

        let source_height = data_len / crate::IMG_SIZE;

        if start_x >= crate::IMG_SIZE
            || start_z >= source_height
            || start_x >= self.logical_width
            || start_z >= self.logical_height
        {
            return None;
        }

        let width = x_size
            .min(crate::IMG_SIZE - start_x)
            .min(self.logical_width - start_x);

        let height = z_size
            .min(source_height - start_z)
            .min(self.logical_height - start_z);

        Some((width, height))
    }
    // Source Width Unnecesary TODO: Remove
    pub fn copy_u32_color_data(
        &mut self,
        img_data: &[u32],
        start_x: usize,
        start_z: usize,
        x_size: usize,
        z_size: usize,
    ) {
        let Some((width, height)) = self.get_copy_region(
            img_data.len(), start_x, start_z, x_size, z_size
        ) else {
            return;
        };

        for z in 0..height {
            let src_start = (start_z + z) * crate::IMG_SIZE + start_x;
            let dst_start = (start_z + z) * self.logical_width + start_x;

            self.pixels[dst_start..dst_start + width]
                .copy_from_slice(&img_data[src_start..src_start + width]);
        }

        self.mark_dirty_region(start_x, start_z, width, height);
    }
    // Source Width Unnecesary TODO: Remove
    pub fn copy_u16_greyscale_data(
        &mut self,
        img_data: &[u16],
        start_x: usize,
        start_z: usize,
        x_size: usize,
        z_size: usize,
    ) {
        let Some((width, height)) = self.get_copy_region(
            img_data.len(), start_x, start_z, x_size, z_size
        ) else {
            return;
        };

        for z in 0..height {
            let src_start = (start_z + z) * crate::IMG_SIZE + start_x;
            let dst_start = (start_z + z) * self.logical_width + start_x;

            for x in 0..width {
                let gray = (img_data[src_start + x] >> 8) as u32;

                self.pixels[dst_start + x] =
                    (gray << 24) | (gray << 16) | (gray << 8) | 0xFF;
            }
        }

        self.mark_dirty_region(start_x, start_z, width, height);
    }
    // Source Width Unnecesary TODO: Remove
    pub fn copy_u8_greyscale_data(
        &mut self,
        img_data: &[u8],
        start_x: usize,
        start_z: usize,
        x_size: usize,
        z_size: usize,
    ) {
        let Some((width, height)) = self.get_copy_region(
            img_data.len(), start_x, start_z, x_size, z_size
        ) else {
            return;
        };

        for z in 0..height {
            let src_start = (start_z + z) * crate::IMG_SIZE + start_x;
            let dst_start = (start_z + z) * self.logical_width + start_x;

            for x in 0..width {
                let gray = img_data[src_start + x] as u32;

                self.pixels[dst_start + x] =
                    (gray << 24) | (gray << 16) | (gray << 8) | 0xFF;
            }
        }

        self.mark_dirty_region(start_x, start_z, width, height);
    }

     fn logical_to_subwindow_coords(&self, x: usize, z: usize, ui: &egui::Ui) -> egui::Pos2{
        let display_size = egui::vec2(
            self.visual_width as f32,
            self.visual_height as f32,
        );

        let uv_min = egui::vec2(
            self.zoom_data.1 - self.zoom_data.0 / 2.0,
            self.zoom_data.2 - self.zoom_data.0 / 2.0,
        );

        let uv_max = egui::vec2(
            self.zoom_data.1 + self.zoom_data.0 / 2.0,
            self.zoom_data.2 + self.zoom_data.0 / 2.0,
        );

        let uv_pos = egui::vec2(
            (x as f32 + 0.5) / self.logical_width as f32,
            (z as f32 + 0.5) / self.logical_height as f32,
        );

        let normalized = egui::vec2(
            (uv_pos.x - uv_min.x) / (uv_max.x - uv_min.x),
            (uv_pos.y - uv_min.y) / (uv_max.y - uv_min.y),
        );

        self.parent_window_rect.min + normalized * display_size
    }

    fn bounding_rect(&self) -> Rect{
        egui::Rect::from_min_size(
            self.parent_window_rect.min,
            egui::vec2(self.visual_width as f32, self.visual_height as f32),
        )
    }

    fn logical_to_visual_size(&self, n: f32) -> f32{
        n / self.zoom_data.0 / (self.logical_height as f32 / self.visual_height as f32)
    }

    pub fn overlay_dot(&mut self, x: usize, z: usize) {
        self.overlay_queue.push(OverlayShape::Dot { x, z });
    }

    pub fn overlay_hollow_circle(&mut self, x: usize, z: usize) {
        self.overlay_queue.push(OverlayShape::HollowCircle {
            x,
            z,
            radius: 100.0,
        });
    }

    pub fn overlay_pixel(&mut self, x: usize, z: usize) {
        self.overlay_queue.push(OverlayShape::Pixel { x, z });
    }

    pub fn show_texture(&mut self, ui: &mut egui::Ui) {

        self.send_texture_to_gpu_internal(ui);

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
                    .uv(uv)
                    .sense(egui::Sense::click_and_drag()),
            );

            self.parent_window_rect = response.rect;

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

        let painter = ui.painter().with_clip_rect(self.bounding_rect());

        // Take ownership of the queue and empty the original.
        let overlays = std::mem::take(&mut self.overlay_queue);

        for overlay in overlays {
            match overlay {
                OverlayShape::Dot { x, z } => {
                    painter.circle_filled(
                        self.logical_to_subwindow_coords(x, z, ui),
                        1.5,
                        egui::Color32::RED,
                    );
                }

                OverlayShape::HollowCircle { x, z, radius } => {
                    painter.circle_stroke(
                        self.logical_to_subwindow_coords(x, z, ui),
                        self.logical_to_visual_size(radius),
                        egui::Stroke::new(2.0, egui::Color32::RED),
                    );
                }

                OverlayShape::Pixel { x, z } => {
                    painter.rect_filled(
                        egui::Rect::from_center_size(
                            self.logical_to_subwindow_coords(x, z, ui),
                            egui::vec2(
                                self.logical_to_visual_size(1.0).max(2.0),
                                self.logical_to_visual_size(1.0).max(2.0),
                            ),
                        ),
                        0.0,
                        egui::Color32::RED,
                    );
                }
            }
        }
    }

    pub fn handle_zooming(&mut self, scroll_y: f32){
        if self.hover_pos.is_some() {

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
    pub shaded_relief_map: Image,
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

            shaded_relief_map: Image::new(
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
        self.shaded_relief_map.zoom_data = data;

        self.t_map.zoom_data = data;
        self.h_map.zoom_data = data;
        self.c_map.zoom_data = data;
        self.e_map.zoom_data = data;
        self.w_map.zoom_data = data;

    }
}
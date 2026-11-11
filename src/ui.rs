use egui_dock::tab_viewer::OnCloseResponse;

use crate::wrapper::MapGenerator;
use crate::image_util::{create_shaded_relief_map, u16_to_float};
use crate::image::Images;

pub struct TabViewer<'a> {
    pub images: &'a mut Images,
    pub map_generator: &'a mut MapGenerator,
    pub hover_pos: &'a mut Option<(usize, usize)>,
}
                
impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {

        if tab == "Main" || tab == "Heightmap" || tab == "Shaded Relief" {
            let active_map = if tab == "Main" {
                &mut self.images.main_map
            } else if tab == "Shaded Relief" {
                &mut self.images.shaded_relief_map
            } else {
                &mut self.images.height_map
            };


            if active_map.hover_pos.is_some() {
                let (x, z) = active_map.hover_pos.unwrap();
                let uv: (f32, f32) = active_map.mouse_pos_uv.unwrap();

                if (0.0..=1.0).contains(&uv.0) && (0.0..=1.0).contains(&uv.1) {
                    *self.hover_pos = Some((x as usize, z as usize));
                } else {
                    *self.hover_pos = None;
                }
            }
            else {
                *self.hover_pos = None;
            }

            let scroll = ui.input(|i| i.smooth_scroll_delta) * 2.0;

            active_map.handle_zooming(scroll.y);
            active_map.show_texture(ui);

            if let Some((x, z)) = *self.hover_pos{
                active_map.overlay_hollow_circle(ui, x, z);

                let mouse_pressed = ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));

                if mouse_pressed{
                    self.map_generator.reduce_continental_circle(x, z, 100.0, 0.01, 1.0);
                    self.images.e_map.copy_u16_greyscale_data(&self.map_generator.e_map);
                }
            }

            let zoom_data = active_map.zoom_data;

            if let Some((x, z)) = *self.hover_pos{
                let in_game_coords: (i32, i32) = (
                    ((x as f32 - crate::IMG_SIZE as f32 / 2.0) * 4.0) as i32,
                    ((z as f32 - crate::IMG_SIZE as f32 / 2.0) * 4.0) as i32,
                );   

                //ui.label(format!("Pixel: ({}, {})", x as i32, z as i32));
                //ui.label(format!("UV Coords: ({}, {})", uv.0, uv.1));
                ui.label(format!(
                    "In Game Coords: ({}, {})",
                    in_game_coords.0,
                    in_game_coords.1
                ));   


                if let Some((x_int, z_int)) = *self.hover_pos{
                    ui.label(format!(
                        "Terrain Height: {}",
                        self.map_generator.height_map[z_int * crate::IMG_SIZE as usize + x_int]
                    ));   
                }

            }

            self.images.set_zoom_data(zoom_data);
               
            ui.heading("Settings:");

            

            if ui.button("Regenerate").clicked(){
                self.map_generator.gen_biomes_multithreaded(1024);
                self.images.main_map.copy_u32_color_data(&self.map_generator.main_map);
                self.images.height_map.copy_u8_greyscale_data(&self.map_generator.height_map);
                self.images.t_map.copy_u16_greyscale_data(&self.map_generator.t_map);
                self.images.h_map.copy_u16_greyscale_data(&self.map_generator.h_map);
                self.images.c_map.copy_u16_greyscale_data(&self.map_generator.c_map);
                self.images.e_map.copy_u16_greyscale_data(&self.map_generator.e_map);
                self.images.w_map.copy_u16_greyscale_data(&self.map_generator.w_map);

                create_shaded_relief_map(
                &mut self.images.shaded_relief_map, 
                &self.map_generator.main_map, 
                &self.map_generator.height_map, 
                crate::IMG_SIZE, crate::IMG_SIZE);
        
                self.images.shaded_relief_map.send_texture_to_gpu();

            }         

            if ui.button("Reset UV").clicked(){
                self.images.set_zoom_data((1.0, 0.5, 0.5));
            }      
        }

        else if tab == "Temperature" {
            self.images.t_map.show_texture(ui);
            if let Some((x, z)) = *self.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(self.map_generator.t_map[z * crate::IMG_SIZE as usize + x])));
                self.images.t_map.overlay_pixel(ui, x, z);
            }
        } 
        else if tab == "Humidity" {
            self.images.h_map.show_texture(ui);
            if let Some((x, z)) = *self.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(self.map_generator.h_map[z * crate::IMG_SIZE as usize + x])));
                self.images.h_map.overlay_pixel(ui, x, z);
            }
        } 
        else if tab == "Continental" {
            self.images.c_map.show_texture(ui);
            if let Some((x, z)) = *self.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(self.map_generator.c_map[z * crate::IMG_SIZE as usize + x])));
                self.images.c_map.overlay_pixel(ui, x, z);
            }
        } 
        else if tab == "Erosion" {
            self.images.e_map.show_texture(ui);
            if let Some((x, z)) = *self.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(self.map_generator.e_map[z * crate::IMG_SIZE as usize + x])));
                self.images.e_map.overlay_pixel(ui, x, z);
            }
        } 
        else if tab == "Weirdness" {
            self.images.w_map.show_texture(ui);
            if let Some((x, z)) = *self.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(self.map_generator.w_map[z * crate::IMG_SIZE as usize + x])));
                self.images.w_map.overlay_pixel(ui, x, z);
            }
        } 
        else {
            ui.label("Unknown Tab");
        }
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> OnCloseResponse {
        OnCloseResponse::Ignore
    }
}
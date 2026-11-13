use egui_dock::tab_viewer::OnCloseResponse;
use crate::appstate::{AppState};
use crate::enums::{MainMap};
use crate::image_util::{u16_to_float};

pub struct TabViewer<'a> {
    pub appstate: &'a mut AppState,
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

        let aps = &mut self.appstate;

        if tab == "Main" || tab == "Heightmap" || tab == "Shaded Relief" {

            aps.active_map = if tab == "Main" {
                MainMap::Main
            } else if tab == "Shaded Relief" {
                MainMap::ShadedRelief
            } else {
                MainMap::Heightmap
            };


            if aps.active_map_mut().hover_pos.is_some() {
                let (x, z) = aps.active_map_mut().hover_pos.unwrap();
                let uv: (f32, f32) = aps.active_map_mut().mouse_pos_uv.unwrap();

                if (0.0..=1.0).contains(&uv.0) && (0.0..=1.0).contains(&uv.1) {
                    aps.hover_pos = Some((x as usize, z as usize));
                } else {
                    aps.hover_pos = None;
                }
            }
            else {
                aps.hover_pos = None;
            }

            let scroll = ui.input(|i| i.smooth_scroll_delta) * 2.0;

            aps.active_map_mut().handle_zooming(scroll.y);
            aps.active_map_mut().show_texture(ui);

            if let Some((x, z)) = aps.hover_pos {
                aps.active_map_mut().overlay_hollow_circle(x, z);
                aps.active_noisemap_img_mut().overlay_hollow_circle(x, z);


                // On left click will increase noisemap values
                // On right click will decrease noisemap values
                // On both clicked simultaenously will do nothing

                let mut draw_multiplier = 0;

                if ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary)) {
                    draw_multiplier = 1;
                }

                if ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary)) {
                    draw_multiplier -= 1;
                }

                if draw_multiplier != 0 {
                    let radius: usize = 100;
                    let map_size = crate::IMG_SIZE as usize;


                    aps.map_generator.modify_noise_data_in_circle(
                        aps.active_noise_map, x, z, radius as f32, 0.01 * draw_multiplier as f32, 1.0
                    );

                    let start_x = x.saturating_sub(radius);
                    let start_z = z.saturating_sub(radius);

                    let end_x = x.saturating_add(radius).min(map_size);
                    let end_z = z.saturating_add(radius).min(map_size);

                    let width = end_x - start_x;
                    let height = end_z - start_z;

                    let (img, data) = aps.active_noisemap_dual_refs();

                    img.copy_u16_greyscale_data(data, start_x, start_z, width, height);
                }
            }

            let zoom_data = aps.active_map_mut().zoom_data;

            if let Some((x, z)) = aps.hover_pos{
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


                if let Some((x_int, z_int)) = aps.hover_pos{
                    ui.label(format!(
                        "Terrain Height: {}",
                        aps.map_generator.height_map[z_int * crate::IMG_SIZE as usize + x_int]
                    ));   
                }

            }

            aps.images.set_zoom_data(zoom_data);
               
            ui.heading("Settings:");

            if ui.button("Regenerate").clicked(){
                aps.update_maps();
            } 

            if ui.button("Reset UV").clicked(){
                aps.images.set_zoom_data((1.0, 0.5, 0.5));
            }      
        }

        else if tab == "Temperature" {
            aps.images.t_map.show_texture(ui);
            if let Some((x, z)) = aps.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(aps.map_generator.t_map[z * crate::IMG_SIZE + x])));
                aps.images.t_map.overlay_pixel(x, z);
            }
        } 
        else if tab == "Humidity" {
            aps.images.h_map.show_texture(ui);
            if let Some((x, z)) = aps.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(aps.map_generator.h_map[z * crate::IMG_SIZE + x])));
                aps.images.h_map.overlay_pixel(x, z);
            }
        } 
        else if tab == "Continental" {
            aps.images.c_map.show_texture(ui);
            if let Some((x, z)) = aps.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(aps.map_generator.c_map[z * crate::IMG_SIZE + x])));
                aps.images.c_map.overlay_pixel(x, z);
            }
        } 
        else if tab == "Erosion" {
            aps.images.e_map.show_texture(ui);
            if let Some((x, z)) = aps.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(aps.map_generator.e_map[z * crate::IMG_SIZE + x])));
                aps.images.e_map.overlay_pixel(x, z);
            }
        } 
        else if tab == "Weirdness" {
            aps.images.w_map.show_texture(ui);
            if let Some((x, z)) = aps.hover_pos{
                ui.label(format!("Val: {:.2}", u16_to_float(aps.map_generator.w_map[z * crate::IMG_SIZE + x])));
                aps.images.w_map.overlay_pixel(x, z);
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
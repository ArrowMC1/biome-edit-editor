#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


static IMG_SIZE: i32 = 2048;

use eframe::{NativeOptions, egui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::image_util::{create_shaded_relief_map};
use crate::wrapper::*;
use crate::image::Images;

use crate::ui::TabViewer;

use std::time::Instant;

mod wrapper;
mod image;
mod image_util;
mod ui;


fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    tree: DockState<String>,
    images: Images,
    map_generator: MapGenerator,
    hover_pos: Option<(usize, usize)>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Main".to_owned(), "Shaded Relief".to_owned(), "Heightmap".to_owned()]);

        let mut images: Images = Images::new(800, 800, 300, 300, IMG_SIZE as usize, IMG_SIZE as usize);

        let mut map_generator = MapGenerator::new(IMG_SIZE, IMG_SIZE);

        let mut hover_pos = None;

        let start = Instant::now();

        set_global_write_mode(true);
        set_global_shift_noise(false);
        map_generator.gen_biomes_multithreaded(1024);
        set_global_shift_noise(true);
        set_global_write_mode(false);

        let duration = start.elapsed();
        println!("Biome Generation Took: {:?}ms", duration.as_millis());


        for value in &mut map_generator.h_map {
            //*value = float_to_u16(u16_to_float(*value) - 0.5);
        }

        images.main_map.copy_u32_color_data(&map_generator.main_map);
        images.height_map.copy_u8_greyscale_data(&map_generator.height_map);
        images.t_map.copy_u16_greyscale_data(&map_generator.t_map);
        images.h_map.copy_u16_greyscale_data(&map_generator.h_map);
        images.c_map.copy_u16_greyscale_data(&map_generator.c_map);
        images.e_map.copy_u16_greyscale_data(&map_generator.e_map);
        images.w_map.copy_u16_greyscale_data(&map_generator.w_map);

        create_shaded_relief_map(
            &mut images.shaded_relief_map, 
            &map_generator.main_map, 
            &map_generator.height_map, 
            IMG_SIZE, IMG_SIZE);
        
        images.shaded_relief_map.send_texture_to_gpu();



        //let [_, _b1] = tree
        //    .main_surface_mut()
        //    .split_below(NodeIndex::root(), 9.0/10.0, vec!["Settings".to_owned()]);


        let [_, t2] =
            tree.main_surface_mut()
            .split_right(NodeIndex::root(), 1.0/2.0, vec!["Continental".to_owned()]);
        
        let [_, _b2] = tree
            .main_surface_mut()
            .split_below(t2, 1.0/2.0, vec!["Temperature".to_owned()]);


        let [_, t3] =
            tree.main_surface_mut()
            .split_right(t2, 1.0/3.0, vec!["Erosion".to_owned()]);
        
        let [_, _b3] = tree
            .main_surface_mut()
            .split_below(t3, 1.0/2.0, vec!["Humidity".to_owned()]);


        let [_, t4] =
            tree.main_surface_mut()
            .split_right(t3, 1.0/2.0, vec!["Weirdness".to_owned()]);
        
        let [_, _b4] = tree
            .main_surface_mut()
            .split_below(t4, 1.0/2.0, vec!["Peaks & Valleys".to_owned()]);

        Self {tree, images, map_generator, hover_pos}

    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().request_repaint_after_secs(1.0 / 240.0);
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_close_buttons(false)
            .draggable_tabs(false)
            .show_leaf_close_all_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_inside(ui, &mut TabViewer { 
                images: &mut self.images, 
                map_generator: &mut self.map_generator, 
                hover_pos: &mut self.hover_pos,
            });
    }
}
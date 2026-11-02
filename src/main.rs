#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{NativeOptions, egui};
use egui_dock::tab_viewer::OnCloseResponse;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::wrapper::*;

use std::time::Instant;

mod wrapper;
mod image;

use image::Image;
use image::Images;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
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

struct TabViewer<'a> {
    images: &'a mut Images,
    map_generator: &'a mut MapGenerator,
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
        if tab == "Main" {
            self.images.main_map.show_texture(ui);
        }

        else if tab == "Temperature" {
            self.images.t_map.show_texture(ui);
        } 
        else if tab == "Humidity" {
            self.images.h_map.show_texture(ui);
        } 
        else if tab == "Continental" {
            self.images.c_map.show_texture(ui);
        } 
        else if tab == "Erosion" {
            self.images.e_map.show_texture(ui);
        } 
        else if tab == "Weirdness" {
            self.images.w_map.show_texture(ui);
        } 
        
        else if tab == "Settings" {
            
            if ui.button("Regenerate").clicked(){
                self.map_generator.gen_biomes_multithreaded(32);
                self.images.main_map.copy_u32_color_data(&self.map_generator.main_map);
                self.images.t_map.copy_u16_greyscale_data(&self.map_generator.t_map);
                self.images.h_map.copy_u16_greyscale_data(&self.map_generator.h_map);
                self.images.c_map.copy_u16_greyscale_data(&self.map_generator.c_map);
                self.images.e_map.copy_u16_greyscale_data(&self.map_generator.e_map);
                self.images.w_map.copy_u16_greyscale_data(&self.map_generator.w_map);

            }

        } else {
            ui.label(format!("Content of {tab}"));
        }
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> OnCloseResponse {
        OnCloseResponse::Ignore
    }
}

struct MyApp {
    tree: DockState<String>,
    images: Images,
    map_generator: MapGenerator
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Main".to_owned()]);

        let mut images: Images = Images::new(800, 800, 300, 300, 2048, 2048);

        let mut map_generator = MapGenerator::new(2048, 2048);

        let start = Instant::now();

        set_global_write_mode(true);
        set_global_shift_noise(false);
        map_generator.gen_biomes_multithreaded(32);
        set_global_shift_noise(true);
        set_global_write_mode(false);

        let duration = start.elapsed();
        println!("Biome Generation Took: {:?}ms", duration.as_millis());


        //for value in &mut map_generator.c_map {
        //    *value = value.saturating_mul(2);
        //}

        images.main_map.copy_u32_color_data(&map_generator.main_map);
        images.t_map.copy_u16_greyscale_data(&map_generator.t_map);
        images.h_map.copy_u16_greyscale_data(&map_generator.h_map);
        images.c_map.copy_u16_greyscale_data(&map_generator.c_map);
        images.e_map.copy_u16_greyscale_data(&map_generator.e_map);
        images.w_map.copy_u16_greyscale_data(&map_generator.w_map);



        let [_, _b1] = tree
            .main_surface_mut()
            .split_below(NodeIndex::root(), 9.0/10.0, vec!["Settings".to_owned()]);


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

        Self {tree, images, map_generator}

    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut TabViewer { images: &mut self.images, map_generator: &mut self.map_generator });
    }
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{NativeOptions, egui};
use egui_dock::tab_viewer::OnCloseResponse;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::wrapper::*;

use std::time::Instant;

mod wrapper;
mod image;

use image::Image;

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
    main_map: &'a mut Image,
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
            self.main_map.show_texture(ui);
        } else if tab == "Settings" {
            
            if ui.button("Regenerate").clicked(){
                //gen_biomes_multithreaded(2048, 2048, 32);     
                self.main_map.send_texture_to_gpu();       
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
    main_map: Image,
    map_generator: MapGenerator
}

impl Default for MyApp {
    //splits into 8 windows t1 being top 1 going till t4 on the right, same with bottom windows as
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Main".to_owned()]);

        let mut main_map: Image = Image::new(800, 800, 2048, 2048);

        let mut map_generator = MapGenerator::new(2048, 2048);


        let start = Instant::now();
        map_generator.gen_biomes_multithreaded(32);
        let duration = start.elapsed();
        println!("Biome Generattion Took: {:?}ms", duration.as_millis());

        main_map.copy_u32_color_data(&map_generator.main_map);

        //t1 is main

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

        Self {tree, main_map, map_generator}

    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut TabViewer { main_map: &mut self.main_map, map_generator: &mut self.map_generator });
    }
}
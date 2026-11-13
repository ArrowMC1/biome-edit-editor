#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


static IMG_SIZE: usize = 2048;

use eframe::{NativeOptions, egui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use crate::appstate::AppState;

use crate::ui::TabViewer;

use std::time::Instant;

mod wrapper;
mod image;
mod image_util;
mod ui;
mod appstate;
mod enums;


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
    appstate: AppState
}

impl Default for MyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Main".to_owned(), "Shaded Relief".to_owned(), "Heightmap".to_owned()]);

        let mut appstate = AppState::default();

        let start = Instant::now();
        appstate.initialise_maps();
        let duration = start.elapsed();
        println!("Biome Generation Took: {:?}ms", duration.as_millis());



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

        Self {tree, appstate}

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
                appstate: &mut self.appstate, 
            });
    }
}
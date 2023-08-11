#![windows_subsystem = "windows"]

use std::sync::{Arc, Mutex};
use egui::{Pos2, Color32, Stroke, Visuals};

mod wooting;
mod windows;

fn main() {
    let wdata = wooting::initialise_wooting().unwrap();
    let wdata2 = Arc::clone(&wdata);


    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native("Wooting Overlay", options, Box::new(|_cc| {
        Box::new(WootingApp::new(wdata2))
    })).unwrap();

    {
        let mut lock = wdata.lock().unwrap();
        lock.end();
    }
}

struct WootingApp {
    data: Arc<Mutex<wooting::WootingData>>,
}

impl WootingApp {
    fn new(data: Arc<Mutex<wooting::WootingData>>) -> Self {
        Self { data }
    }
}

impl eframe::App for WootingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::containers::Frame {
            fill: Color32::TRANSPARENT,
            stroke: Stroke::NONE,
            ..Default::default()
        };
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            
            let painter = ui.painter();

            let (data, head) = {
                let lock = self.data.lock().unwrap();
                (lock.copy_data(), lock.get_head())
            };

            let length = data.len() as i32;
            let sample_offset = 500.0 / length as f32;
            for i in 0..length {
                let idx = (i + head) % length;
                let lidx = (i + head - 1 + length) % length;

                if i == 1 { continue; }

                let lentry = &data[lidx as usize];
                let centry = &data[idx as usize];

                let points = vec![
                    Pos2::new((1.0 - lentry.k1) * 100.0, (i - 1) as f32 * sample_offset), 
                    Pos2::new(100.0, (i - 1) as f32 * sample_offset),
                    Pos2::new(100.0, i as f32 * sample_offset),
                    Pos2::new((1.0 - centry.k1) * 100.0, i as f32 * sample_offset)];

                let col = match centry.k1_down {
                    false => Color32::LIGHT_GRAY,
                    true => Color32::LIGHT_GREEN,
                };
                let path = egui::Shape::convex_polygon(points, col, Stroke::NONE);
                painter.add(path);

                let points = vec![
                    Pos2::new(lentry.k2 * 100.0 + 100.0, (i - 1) as f32 * sample_offset), 
                    Pos2::new(100.0, (i - 1) as f32 * sample_offset),
                    Pos2::new(100.0, i as f32 * sample_offset),
                    Pos2::new(centry.k2 * 100.0 + 100.0, i as f32 * sample_offset)];

                let col = match centry.k2_down {
                    false => Color32::LIGHT_GRAY,
                    true => Color32::LIGHT_GREEN,
                };
                let path = egui::Shape::convex_polygon(points, col, Stroke::NONE);
                painter.add(path);
            }
        });

        ctx.request_repaint();
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
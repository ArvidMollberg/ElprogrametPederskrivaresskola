use egui::{Key, Rect, Vec2};
mod render;
mod sim;

// grid resolution — one cell covers this many screen pixels on a side.
// 1 pixel/cell would make a huge atom count (and a very deep push-chain
// recursion) for a normal window, so cells are coarser than pixels
pub const CELL_SIZE: f32 = 4.0;
fn main() {
    eframe::run_native(
        "RC-4",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
}
#[derive(Default)]
pub struct atom{
    vel: Vec2,
    mat: usize,
    pos: Vec2,
    tmp: f32,
    occ: bool,
}

// data shape only — the operations on it (new/swap/indexing) live in sim.rs
#[derive(Default)]
pub struct Grid {
    cells: Vec<atom>,
    pub(crate) cols: usize,
    pub(crate) rows: usize,
}

#[derive(Default)]
pub struct MyApp {
    pub(crate) selected: u32,
    pub(crate) size: u32,
    pub(crate) grid: Grid,
}


impl eframe::App for MyApp {
    // this runs once per frame, redraws the whole window
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // track each widget's own rect so painting can avoid exactly them,
        // not the whole toolbar row (including the gaps between them)
        let mut button_rects: Vec<Rect> = Vec::new();
        ui.horizontal(|ui| {
            // button() draws it and returns a Response; clicked() is true for
            // exactly the one frame the click happened on
            let r = ui.button("Nill");
            if r.clicked() { self.selected = 0; }

            let r = ui.button("Sand");
            if r.clicked() { self.selected = 5; }
            button_rects.push(r.rect);

            let r = ui.button("Stone");
            if r.clicked() { self.selected = 1; }
            button_rects.push(r.rect);

            let r = ui.button("Water");
            if r.clicked() { self.selected = 2; }
            button_rects.push(r.rect);

            let r = ui.button("Bedrock");
            if r.clicked() { self.selected = 3; }
            button_rects.push(r.rect);

            let r = ui.button("Lava");
            if r.clicked() { self.selected = 4; }
            button_rects.push(r.rect);

            let r = ui.add(egui::Slider::new(&mut self.size, 1..=30));
            button_rects.push(r.rect);
        });

    
        
        // Claim remaining window space for canvas
        let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter_at(rect);    
        
        let mmb = ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Middle));
        let rmb = ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary));
        // held, not just clicked, so dragging paints a continuous stroke
        let lmb = ui.input(|i| i.pointer.primary_down());
        let rkey = ui.input(|i| i.key_pressed(Key::R));
        let ckey = ui.input(|i| i.key_pressed(Key::C));
        let vkey = ui.input(|i| i.key_pressed(Key::V));
        let mouse_pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(rect.center());
        let work_zone = {rect.shrink(5.0)};

        // step first so the grid exists/is correctly sized before painting
        // into it, then paint, then draw whatever the grid looks like now
        self.step(ui, work_zone);
        if lmb {
            self.paint(mouse_pos, work_zone);
        }
        render::draw_canvas(ui, &self.selected, &self.size, &rect, &button_rects, &self.grid);
    }
}

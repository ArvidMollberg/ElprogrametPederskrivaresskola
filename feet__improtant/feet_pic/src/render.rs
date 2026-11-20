use egui::{Color32, Rect, Stroke, StrokeKind, Ui};
use crate::{Grid, CELL_SIZE};

// material index -> display color. None means "don't draw this cell"
// (mat 0 / "Nill" is empty) — keep in sync with the button indices in main.rs
fn material_color(mat: usize) -> Option<Color32> {
    match mat {
        0 => None,                                   // Nill — empty
        1 => Some(Color32::from_rgb(130, 130, 130)),  // Stone
        2 => Some(Color32::from_rgb(40, 110, 220)),   // Water
        3 => Some(Color32::from_rgb(60, 60, 60)),     // Bedrock
        4 => Some(Color32::from_rgb(230, 90, 20)),    // Lava
        5 => Some(Color32::from_rgb(210, 190, 110)),  // Sand
        _ => None,
    }
}

pub fn draw_canvas(ui: &mut Ui, selected: &u32, size: &u32, rect: &Rect, button_rects: &[Rect], grid: &Grid) {
    let work_zone: Rect = {rect.shrink(5.0)};
    let mouse_pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(ui.min_rect().center());

    // Render brush selection preview
    let brush_radius = *size as f32;
    // bounding box of the circle itself, not just its center point
    let brush_bounds = Rect::from_center_size(mouse_pos, egui::vec2(brush_radius * 2.0, brush_radius * 2.0));
    
    
    ui.painter().rect(*rect, 0.0, Color32::BLACK, Stroke::new(50.0_f32, Color32::from_rgba_unmultiplied(70, 70, 70, 255)), StrokeKind::Inside);
    ui.painter().rect(work_zone, 0.0, Color32::BLACK, Stroke::new(5.0_f32, Color32::GRAY), StrokeKind::Outside);

    // one filled square per occupied cell — this IS the powder itself.
    // work_zone.min is the same origin paint()/declare() size the grid
    // against, so grid (0,0) lines up with the top-left of work_zone
    for y in 0..grid.rows {
        for x in 0..grid.cols {
            if let Some(color) = material_color(grid[(x, y)].mat) {
                let cell_rect = Rect::from_min_size(
                    work_zone.min + egui::vec2(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE),
                    egui::vec2(CELL_SIZE, CELL_SIZE),
                );
                ui.painter().rect_filled(cell_rect, 0.0, color);
            }
        }
    }

    let blocked = button_rects.iter().any(|r| brush_bounds.intersects(*r));
    if rect.contains(mouse_pos) && !blocked {
        // selected == 0 ("Nill") is the eraser — show that differently so
        // it doesn't read as "about to place a solid white block"
        let preview_color = if *selected == 0 { Color32::WHITE } else { material_color(*selected as usize).unwrap_or(Color32::WHITE) };
        ui.painter().circle_stroke(mouse_pos, brush_radius, Stroke::new(1.0_f32, preview_color));
    }
}//how do i deform the circle to the bounding box work_zone. and i want to skip the basic logic which stops making circle as soon as its inside the buttons claims. rather id like it to deform to bounding box so skipping to paint the parts outside the bounding box.
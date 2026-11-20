use egui::{Key::J, Pos2, Rect, Ui, Vec2};
use crate::{MyApp, Grid, atom, CELL_SIZE};

const GRAVITY: f32 = 0.15;      // added to vel.y every step — builds up like real acceleration
const WALL_DAMPING: f32 = 0.5;  // fraction of velocity KEPT after hitting the actual grid edge
const PUSH_TRANSFER: f32 = 0.6; // fraction of velocity handed to whatever a block bumps into

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut cells = Vec::with_capacity(cols * rows);
        for _ in 0..(cols * rows) {
            cells.push(atom::default());
        }
        Self { cells, cols, rows }
    }

    // move the atom at `from` into `to`, leaving `from` as atom::default()
    // (mat == 0, "Nill") behind — fails if `to` is already occupied
    pub fn try_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        if self[to].mat != 0 {
            return false;
        }
        let moving = std::mem::take(&mut self[from]);
        self[to] = moving;
        true
    }

    // tries to shove whatever is at `pos` one cell further in `dir`. if
    // something already occupies that cell, push IT forward first
    // (recursively) — so a whole stack of blocks moves together. returns
    // false only when the chain is stopped cold by the edge of the grid,
    // in which case everything in the chain loses velocity (WALL_DAMPING)
    fn push(&mut self, pos: (usize, usize), dir: (i32, i32), incoming_vel: Vec2) -> bool {
        let nx = pos.0 as i32 + dir.0;
        let ny = pos.1 as i32 + dir.1;
        if nx < 0 || ny < 0 || nx as usize >= self.cols || ny as usize >= self.rows {
            self[pos].vel = incoming_vel * WALL_DAMPING;
            return false; // this is the actual wall
        }
        let next = (nx as usize, ny as usize);
        if self[next].mat != 0 {
            let next_vel = self[next].vel;
            if !self.push(next, dir, next_vel + incoming_vel * PUSH_TRANSFER) {
                self[pos].vel *= WALL_DAMPING; // the chain hit a wall — we're stuck too
                return false;
            }
        }
        self.try_move(pos, next);
        self[next].vel = incoming_vel;
        true
    }

    // one simulation tick: gravity pulls every block down, each block then
    // tries to move toward wherever its velocity points (so sideways
    // velocity biases which way it falls), pushing anything in its path
    pub fn apply_gravity_and_move(&mut self) {
        // bottom-to-top so a block moved this frame isn't processed again
        for y in (0..self.rows).rev() {
            for x in 0..self.cols {
                if self[(x, y)].mat == 0 {
                    continue; // empty cell, nothing to simulate
                }

                self[(x, y)].vel.y += GRAVITY;
                let vel = self[(x, y)].vel;

                let dir_x = if vel.x > 0.1 { 1 } else if vel.x < -0.1 { -1 } else { 0 };
                let dir_y = if vel.y > 0.1 { 1 } else if vel.y < -0.1 { -1 } else { 0 };
                if (dir_x, dir_y) == (0, 0) {
                    continue; // not moving yet
                }

                self.push((x, y), (dir_x, dir_y), vel);
            }
        }
    }

    // fill every cell within `radius` cells of `center` with `mat` — this is
    // the actual brush: painting Nill (mat 0) erases, anything else draws
    pub fn stamp(&mut self, center: (i32, i32), radius: i32, mat: usize) {
        let r2 = radius * radius;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy > r2 {
                    continue; // outside the brush's circle
                }
                let (cx, cy) = (center.0 + dx, center.1 + dy);
                if cx < 0 || cy < 0 {
                    continue;
                }
                let (cx, cy) = (cx as usize, cy as usize);
                if cx >= self.cols || cy >= self.rows {
                    continue;
                }
                self[(cx, cy)].mat = mat;
                self[(cx, cy)].vel = Vec2::ZERO; // freshly placed, starts at rest
            }
        }
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = atom;
    fn index(&self, (x, y): (usize, usize)) -> &atom {
        &self.cells[y * self.cols + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut atom {
        &mut self.cells[y * self.cols + x]
    }
}

impl MyApp {
    // only rebuilds when the size actually changed — rebuilding every frame
    // would wipe every atom's position/velocity/material each step
    fn declare(&mut self, wrk_w: u32, wrk_h: u32) {
        let cols = (wrk_w as f32 / CELL_SIZE) as usize;
        let rows = (wrk_h as f32 / CELL_SIZE) as usize;
        if cols != self.grid.cols || rows != self.grid.rows {
            self.grid = Grid::new(cols, rows);
        }
    }

    fn additive(&mut self, ui: &mut Ui){
        let _ = ui;
        self.grid.apply_gravity_and_move();
    }

    // screen position -> grid cell, then stamp the selected material in a
    // circle `size` pixels wide around it. `work_zone` must be the same
    // rect `declare` sized the grid against, so (0,0) lines up correctly
    pub fn paint(&mut self, mouse_pos: Pos2, work_zone: Rect) {
        if !work_zone.contains(mouse_pos) {
            return;
        }
        let gx = ((mouse_pos.x - work_zone.min.x) / CELL_SIZE) as i32;
        let gy = ((mouse_pos.y - work_zone.min.y) / CELL_SIZE) as i32;
        let cell_radius = ((self.size as f32) / CELL_SIZE).max(1.0) as i32;
        self.grid.stamp((gx, gy), cell_radius, self.selected as usize);
    }

    pub fn step(&mut self, ui: &mut Ui, workspace: Rect) {
        let wrk_w = workspace.max.x - workspace.min.x;
        let wrk_h = workspace.max.y - workspace.min.y;
        self.declare(wrk_w.round() as u32, wrk_h.round() as u32);
        self.additive(ui);
    }
}
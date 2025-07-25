use crate::agent::{Agent, Team};
use crate::world::{Pos, World};
use eframe::egui::{self, RichText};
use log::{debug, info};

const WORLD_WIDTH: usize = 30;
const WORLD_HEIGHT: usize = 30;

pub struct App {
    pub world: World,
    pub agents: Vec<Agent>,
}

impl Default for App {
    fn default() -> Self {
        let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);
        world.set_walls(Pos { x: 1, y: 1 });
        world.set_walls(Pos { x: 7, y: 7 });
        world.set_walls(Pos { x: 7, y: 8 });
        world.set_walls(Pos { x: 7, y: 9 });

        info!("World initialized");

        Self {
            world,
            agents: vec![
                Agent::new("Player", Team::Player, Pos { x: 1, y: 0 }),
                Agent::new("Enemy", Team::Enemy, Pos { x: 10, y: 10 }),
            ],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        setup_style(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.heading("ascii tactics");
                ui.add_space(10.0);
                for row in self.render_ascii_grid_colored() {
                    ui.horizontal_wrapped(|ui| {
                        for (ch, color) in row {
                            ui.colored_label(color, RichText::new(ch.to_string()).monospace());
                        }
                    });
                }
                ui.add_space(10.0);
                if ui.button("Tick").clicked() {
                    info!("Tick button pressed");
                    tick_ai(&mut self.agents);
                }
            });
    }
}

impl App {
    fn render_ascii_grid_colored(&self) -> Vec<Vec<(char, egui::Color32)>> {
        let mut rows = Vec::with_capacity(self.world.height);

        for y in 0..self.world.height {
            let mut row = Vec::with_capacity(self.world.width);
            for x in 0..self.world.width {
                let pos = Pos { x, y };
                let (ch, color) = if let Some(agent) = self.agents.iter().find(|a| a.pos == pos) {
                    let color = match agent.team {
                        Team::Player => egui::Color32::from_rgb(100, 200, 255),
                        Team::Enemy => egui::Color32::from_rgb(255, 100, 100),
                    };
                    (agent.glyph(), color)
                } else if self.world.is_walls(pos) {
                    ('#', egui::Color32::GRAY)
                } else {
                    ('.', egui::Color32::DARK_GRAY)
                };
                row.push((ch, color));
            }
            rows.push(row);
        }

        rows
    }
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.override_text_style = Some(egui::TextStyle::Monospace);
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(12.0));
    style.visuals = egui::Visuals::dark();
    style.spacing.item_spacing = egui::vec2(6.0, 0.0);

    ctx.set_style(style);
}

fn tick_ai(agents: &mut [Agent]) {
    let player_pos = agents.iter().find(|a| a.team == Team::Player).unwrap().pos;
    debug!("Player at {:?}", player_pos);

    let enemy = agents.iter_mut().find(|a| a.team == Team::Enemy).unwrap();
    debug!("Enemy at {:?}", enemy.pos);

    if enemy.pos.x < player_pos.x {
        enemy.pos.x += 1;
    } else if enemy.pos.x > player_pos.x {
        enemy.pos.x -= 1;
    } else if enemy.pos.y < player_pos.y {
        enemy.pos.y += 1;
    } else if enemy.pos.y > player_pos.y {
        enemy.pos.y -= 1;
    } else {
        debug!("Enemy reached the player.");
    }

    debug!("Enemy moved to {:?}", enemy.pos);
}

use crate::agent::{Agent, Team};
use crate::world::{Pos, World};
use eframe::egui;

pub struct App {
    pub world: World,
    pub agents: Vec<Agent>,
}

impl Default for App {
    fn default() -> Self {
        let mut world = World::new(5, 3);
        world.set_walls(Pos { x: 1, y: 1 });

        Self {
            world,
            agents: vec![
                Agent::new("Player", Team::Player, Pos { x: 0, y: 0 }),
                Agent::new("Enemy", Team::Enemy, Pos { x: 4, y: 0 }),
            ],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ascii tactics");

            for y in 0..self.world.height {
                let mut line = String::new();
                for x in 0..self.world.width {
                    let pos = Pos { x, y };
                    if let Some(agent) = self.agents.iter().find(|a| a.pos == pos) {
                        line.push(agent.glyph());
                    } else if self.world.is_walls(pos) {
                        line.push('#');
                    } else {
                        line.push('.');
                    }
                }
                ui.label(line);
            }

            if ui.button("Tick").clicked() {
                tick_ai(&mut self.agents);
            }
        });
    }
}

fn tick_ai(agents: &mut Vec<Agent>) {
    let player_pos = agents.iter().find(|a| a.team == Team::Player).unwrap().pos;

    let enemy = agents.iter_mut().find(|a| a.team == Team::Enemy).unwrap();

    if enemy.pos.x < player_pos.x {
        enemy.pos.x += 1;
    } else if enemy.pos.x > player_pos.x {
        enemy.pos.x -= 1;
    } else if enemy.pos.y < player_pos.y {
        enemy.pos.y += 1;
    } else if enemy.pos.y > player_pos.y {
        enemy.pos.y -= 1;
    }
}

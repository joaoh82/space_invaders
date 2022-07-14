use bevy::prelude::Component;
use rand::Rng;

use crate::{WinSize, FORMATION_MEMBERS_MAX, BASE_SPEED};

/// Component - Enemy Formation (per enemy)
#[derive(Clone, Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub speed: f32,
    pub pivot: (f32, f32),
    pub angle: f32, // change per tick
}

/// Resource - Formation Maker
#[derive(Default)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32,
}

/// Formation factory implementation
impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (&self.current_template, self.current_members >= FORMATION_MEMBERS_MAX) {
            // if has current template and still within max members
            (Some(tmpl), false) => {
                self.current_members += 1;
                tmpl.clone()
            }
            // if first formation or previous formation is full (need to create a new one)
            (None, _) | (_, true) => {
                let mut rng = rand::thread_rng();

                // compute the start x/y
                let w_span = win_size.width as f32 / 2.0 + 100.0;
                let h_span = win_size.height as f32 / 2.0 + 100.0;
                let start_x = if rng.gen_bool(0.5) { w_span } else { -w_span };
                let start_y = rng.gen_range(-h_span..h_span) as f32;
                let start = (start_x, start_y);

                // compute the pivot x/y
                let w_span = win_size.width as f32 / 4.;
                let h_span = win_size.height as f32 / 3. + 50.;
                let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

                // compute the radious
                let radius = (rng.gen_range(80.0..150.0), 100.);

                // compute the start angle
                let angle = (start_y - pivot.1).atan2(start_x - pivot.0);

                // speed (fixed for now)
                let speed = BASE_SPEED;

                // create the formation
                let formation = Formation {
                    start,
                    radius,
                    pivot,
                    angle,
                    speed,
                };

                // set the current template
                self.current_template = Some(formation.clone());
                // reset member to 1
                self.current_members = 1;

                formation
            }
        }
    }
}
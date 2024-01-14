use rand::{rngs::ThreadRng, Rng};
use raylib::{prelude::Rectangle, prelude::Vector2};
use std::{
    collections::HashMap,
    ops::Add,
    time::{Duration, SystemTime},
};

use crate::config::Config;

#[derive(Debug, Clone)]
struct Animation {
    frames: Vec<usize>,
    sleep: u64,
    next_animations: Vec<usize>,
    flip_horizontally: bool,
    flip_vertically: bool,
    movement: Option<Vector2>,
}

pub struct StateMachine {
    rng: ThreadRng,
    current_animation_id: usize,
    current_frame: usize,
    next_frame: SystemTime,
    frames: Vec<Rectangle>,
    animations: Vec<Option<Animation>>,
}

impl StateMachine {
    ///generate state machine model from config
    pub fn new(cfg: &Config) -> StateMachine {
        let mut animations: Vec<Option<Animation>> = vec![None; cfg.states.len()];
        let mut frames = Vec::new();

        //generate source rectangles
        let mut frame_table: HashMap<String, usize> = HashMap::new();
        for (i, (name, frame)) in cfg.frames.iter().enumerate() {
            frames.push(Rectangle {
                x: (frame.x * cfg.sprite.width) as f32,
                y: (frame.y * cfg.sprite.height) as f32,
                width: cfg.sprite.width as f32,
                height: cfg.sprite.height as f32,
            });
            frame_table.insert(name.clone(), i);
        }

        //generate animations
        let mut animation_table: HashMap<String, usize> = HashMap::new();
        let mut i = 0;
        fn resolve(
            state_name: &String,
            cfg: &Config,
            animations: &mut Vec<Option<Animation>>,
            frame_table: &HashMap<String, usize>,
            animation_table: &mut HashMap<String, usize>,
            i: &mut usize,
        ) -> usize {
            //check if state has already been visited.
            if let Some(a) = animation_table.get(state_name) {
                return *a;
            }

            //otherwise, try parsing state & animation
            let state = cfg.states.get(state_name).unwrap_or_else(|| {
                panic!("State {} does not exist!", state_name);
            });
            let animation = &cfg.animations.get(&state.animation).unwrap_or_else(|| {
                panic!("animation {} does not exist!", state.animation);
            });

            //mark state as visited
            let current_id = i.clone();
            animation_table.insert(state_name.clone(), current_id);
            *i += 1;

            //add frame indexes to animation
            let mut frames: Vec<usize> = Vec::new();
            for next in &animation.frames {
                frames.push(*frame_table.get(next).unwrap_or_else(|| {
                    panic!("frame {next} does not exist!");
                }))
            }

            //recursively resolve upcoming states
            let mut next_animations = Vec::new();
            for next in &state.next {
                next_animations.push(resolve(
                    next,
                    cfg,
                    animations,
                    frame_table,
                    animation_table,
                    i,
                ));
            }

            //push animation to vec & table
            let movement = if let Some(m) = state.movement {
                Some(Vector2 {
                    x: m.x as f32,
                    y: m.y as f32,
                })
            } else {
                None
            };

            animations[current_id] = Some(Animation {
                frames,
                next_animations,
                sleep: animation.sleep,
                flip_horizontally: state.flip_horizontally,
                flip_vertically: state.flip_vertically,
                movement,
            });
            return current_id;
        }

        resolve(
            &cfg.state,
            cfg,
            &mut animations,
            &frame_table,
            &mut animation_table,
            &mut i,
        );

        StateMachine {
            rng: rand::thread_rng(),
            current_animation_id: 0,
            current_frame: 0,
            next_frame: SystemTime::now(),
            frames,
            animations,
        }
    }

    pub fn update(&mut self) -> Option<Update> {
        //only return update if sleep has elapsed
        if self.next_frame.elapsed().is_ok() {
            let current_animation = self.animations[self.current_animation_id].as_ref().unwrap();

            //update frame counter
            self.current_frame += 1;
            //does this work?
            if self.current_frame >= current_animation.frames.len() {
                self.current_frame = 0;
                let animations = &current_animation.next_animations;
                self.current_animation_id = animations[self.rng.gen_range(0..animations.len())];
            }

            //update next_frame to sleep of current animation
            self.next_frame = SystemTime::now().add(Duration::from_millis(current_animation.sleep));

            //return updated values
            let mut rect = self.frames[current_animation.frames[self.current_frame]].clone();
            if current_animation.flip_horizontally {
                rect.width *= -1.;
            }
    
            if current_animation.flip_vertically {
                rect.height *= -1.;
            }
            return Some(Update {
                delta_position: current_animation.movement,
                frame: rect,
            });
        } else {
            return None;
        }
    }
}

pub struct Update {
    pub delta_position: Option<Vector2>,
    pub frame: Rectangle,
}

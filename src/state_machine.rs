use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::Rectangle;
use std::{
    collections::HashMap,
    ops::Add,
    time::{Duration, SystemTime},
};

use crate::config::Config;

struct Animation {
    frames: Vec<usize>,
    sleep: u64,
    next_animations: Vec<usize>,
}

pub struct StateMachine {
    rng: ThreadRng,
    current_animation: usize,
    current_frame: usize,
    next_frame: SystemTime,
    frames: Vec<Rectangle>,
    animations: Vec<Animation>,
}

impl StateMachine {
    ///generate state machine model from config
    pub fn new(cfg: &Config) -> StateMachine {
        let mut animations = Vec::with_capacity(cfg.animations.len());
        let mut frames = Vec::new();

        //push rectangles to vec & insert new positions into frame table
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

        let mut animation_table: HashMap<String, usize> = HashMap::new();
        let mut i = 0;
        let mut resolve = |state_name: &String| {
            let mut frames: Vec<usize> = Vec::new();
            if let Some(state) = cfg.states.get(state_name) {
                let animation = &cfg.animations.get(&state.animation).unwrap_or_else(|| {
                    panic!("animation {} does not exist!", state.animation);
                });

                //add frame indexes to animation
                for next in &animation.frames {
                    frames.push(*frame_table.get(next).unwrap_or_else(|| {
                        panic!("frame {next} does not exist!");
                    }))
                }

                animations.push(Animation {
                    frames,
                    next_animations: vec![0], //TODO
                    sleep: animation.sleep
                });
                animation_table.insert(state_name.clone(), i);
                i += 1;
            } else {
                panic!("State {state_name} does not exist!");
            }
        };
        resolve(&cfg.state);

        StateMachine {
            rng: rand::thread_rng(),
            current_animation: 0,
            current_frame: 0,
            next_frame: SystemTime::now(),
            frames,
            animations,
        }
    }

    pub fn update(&mut self) {
        if self.next_frame.elapsed().is_ok() {
            self.current_frame += 1;

            if self.current_frame >= 4 {
                self.current_frame = 0;
                let animations = &self.animations[self.current_animation].next_animations;
                self.current_animation = animations[self.rng.gen_range(0..animations.len())];
            }
            self.next_frame = SystemTime::now().add(Duration::from_millis(
                self.animations[self.current_animation].sleep,
            ));
        }
    }

    pub fn get_frame(&self) -> Rectangle {
        self.frames[self.animations[self.current_animation].frames[self.current_frame]]
    }
}

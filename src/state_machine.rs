use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::Rectangle;
use std::{
    ops::Add,
    time::{Duration, SystemTime}, collections::HashMap,
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
                x: frame.x as f32,
                y: frame.y as f32,
                width: cfg.sprite.width as f32,
                height: cfg.sprite.height as f32
            });
            frame_table.insert(name.clone(), i);
        }
        
        let mut animation_table: HashMap<String, usize> = HashMap::new();

        //todo: create animations recursively.
        

        animations.push(Animation {
            frames: Vec::new(),
            next_animations: Vec::new(),
            sleep: 0,
        });


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

            if self.current_frame > self.frames.len() {
                self.current_frame = 0;
                let animations = &self.animations[self.current_animation].next_animations;
                self.current_animation = animations[self.rng.gen_range(0..animations.len())];
                //TODO weights
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

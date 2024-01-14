use config::read_config;
use raylib::prelude::*;
use state_machine::StateMachine;

mod config;
mod state_machine;

fn main() {
    //load config
    let cfg = read_config("./config.yaml");

    //initialize window
    let (mut rl, thread) = raylib::init()
        .size(
            (cfg.sprite.width as f32 * cfg.scale) as i32,
            (cfg.sprite.height as f32 * cfg.scale) as i32,
        )
        .title("Manfred")
        .transparent()
        .undecorated()
        .build();

    rl.set_target_fps(cfg.fps.unwrap_or(30));

    //get screen boundaries
    let max_x = get_monitor_width(get_current_monitor()) as f32;
    let max_y = get_monitor_height(get_current_monitor()) as f32;

    //set initial position
    let mut pos = Vector2 {
        x: cfg.initial_position.x as f32 - cfg.scale*cfg.sprite.width as f32,
        y: cfg.initial_position.y as f32 - cfg.scale*cfg.sprite.height as f32,
    };

    if cfg.initial_position.is_relative {
        pos += Vector2 {
            x: max_x,
            y: max_y
        };
    }
    rl.set_window_position(pos.x as i32, pos.y as i32);

    //loading spritesheet
    let img = {
        match rl.load_texture(&thread, &cfg.texture_path.to_string_lossy()) {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        }
    };

    //create state machine from config
    let mut sm = StateMachine::new(&cfg);

    //drop config for minimal memory usage.
    let scale = cfg.scale;
    let width = cfg.sprite.width;
    let height = cfg.sprite.height;
    drop(cfg);

    //render loop
    let mut source_rec = Rectangle::EMPTY;
    while !rl.window_should_close() {
        if let Some(update) = sm.update() {
            //ensure pet stays on screen & update position
            if let Some(delta) = update.delta_position {
                pos += delta;
                if pos.x + scale*width as f32 + delta.x > max_x {
                    pos.x -= delta.x;
                }
                if pos.y + scale*height as f32 + delta.y > max_y {
                    pos.y -= delta.y;
                }
                
                rl.set_window_position(pos.x as i32, pos.y as i32);
            }

            //update source rect
            source_rec = update.frame;
        }

        //draw frame
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        d.draw_texture_pro(
            &img,
            source_rec,
            Rectangle::new(0., 0., width as f32 * scale, height as f32 * scale),
            Vector2::zero(),
            0.,
            Color::WHITE,
        );
    }
}

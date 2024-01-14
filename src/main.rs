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

    //set initial position
    //TODO: config option
    let mut pos = rl.get_window_position();

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
            //update position
            if let Some(delta) = update.delta_position {
                //TODO: ensure sprite stays on screen
                pos += delta;
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
                Vector2::new(0., 0.),
                0.,
                Color::WHITE,
            );
    }
}

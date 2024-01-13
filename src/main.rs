use config::read_config;
use raylib::prelude::*;
use state_machine::StateMachine;

mod config;
mod state_machine;
#[cfg(test)]
mod tests;

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
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLANK);
        sm.update();

        //draw frame
        d.draw_texture_pro(
            &img,
            sm.get_frame(),
            Rectangle::new(
                0.,
                0.,
                width as f32 * scale,
                height as f32 * scale,
            ),
            Vector2::new(0., 0.),
            0.,
            Color::WHITE,
        );
    }
}

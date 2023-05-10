use raylib::prelude::*;
use std::process::exit;

pub fn draw_menu(rl: &mut RaylibHandle, thread: &RaylibThread, font: &Font, sizex: &i32, sizey: &i32) {
    let logo = rl.load_texture(&thread, "pic/npm.png").unwrap();
    loop {
        if rl.window_should_close() {
            exit(0);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);
        d.gui_set_font(font);
        
        d.draw_texture_ex(&logo, Vector2::new((sizex/2) as f32 - 244.3, (sizey/2) as f32 - 300.0), 0.0, 0.7,Color::WHITE);

        if d.gui_button(Rectangle::new((sizex/2) as f32 - 200.0, (sizey/2) as f32 + 100.0, 400.0, 100.0), Some(rstr!("Quit"))) {
            exit(0);
        }
        if d.gui_button(Rectangle::new((sizex/2) as f32 - 200.0, (sizey/2) as f32 - 50.0, 400.0, 100.0), Some(rstr!("PLAY"))) {
            drop(d);
            super::game::draw(rl, &thread, font, &sizex, &sizey);
        }
    }
}

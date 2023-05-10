use raylib::prelude::*;

pub mod game;
pub mod menu;


fn main() {
    let (mut sizex, mut sizey) = (800, 800);
    let (mut rl, thread) = raylib::init()
        .size(sizex, sizey)
        .title("gaem")
        .resizable()
        .build();

    rl.set_window_min_size(sizex, sizey);
    (sizex, sizey) = (rl.get_screen_width(), rl.get_screen_height());
    rl.set_target_fps(60);
    let poppins = rl.load_font(&thread, "fonts/Poppins-Regular.ttf").unwrap();

    menu::draw_menu(&mut rl, &thread, &poppins, &sizex, &sizey);

}

use raylib::prelude::*;
use std::process::exit;

use self::sub::SubSpawn;

pub mod player;
pub mod sub;

pub fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, font: &Font, sizex: &i32, sizey: &i32) {
    let (x, y): (f32, f32) = ((sizex/2) as f32, (sizey/2) as f32);

    let man_s = rl.load_texture(&thread, "pic/man.png").unwrap();
    let hand_s = rl.load_texture(&thread, "pic/hand.png").unwrap();
    let sub_s = rl.load_texture(&thread, "pic/sub.png").unwrap();

    let mut man = player::Man::new(Vector2::new(x, y), Vector2::zero(), (man_s, hand_s));
    let mut subs = SubSpawn::new(&sub_s);
    loop {
        if rl.window_should_close() {
            exit(0);
        }
        let mut d = rl.begin_drawing(&thread);

        if man.hp <= 0.0 {
            d.draw_text("You died!", (sizex/2) - 200, (sizey/2) - 150, 100, Color::WHITE);
            if d.gui_button(Rectangle::new((sizex/2) as f32 - 200.0, (sizey/2) as f32 - 50.0, 400.0, 100.0), Some(rstr!("Retry"))) {
                man.pos = Vector2::new(x, y);
                man.vel = Vector2::zero();
                man.hp = 1.0;
                subs = SubSpawn::new(&sub_s);
                d.clear_background(Color::DARKGRAY);
                continue
            }
            if d.gui_button(Rectangle::new((sizex/2) as f32 - 200.0, (sizey/2) as f32 + 100.0, 400.0, 100.0), Some(rstr!("Return to menu"))) {
                break
            }

            continue
        }
        d.clear_background(Color::DARKGRAY);
        d.gui_set_font(font);
        d.draw_text(&format!("N: {}", man.n.0), 10, 0, 50, Color::WHITE);
        man = man.update(&mut d, &mut subs);
        d.draw_rectangle_lines_ex(Rectangle::new((sizex - 220) as f32, 10.0, 200.0, 50.0), 5, Color::BLACK);
        d.draw_rectangle_rec(Rectangle::new((sizex - 215) as f32, 15.0, 190.0 * man.hp, 40.0), Color::LIME);
        subs.update(man.pos, d.get_time(),&mut d, &sizex, &sizey);
    }
}



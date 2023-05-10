use raylib::prelude::*;
use super::sub::SubSpawn;
//use raylib::misc::get_random_value;

pub struct Man {
    pub pos: Vector2,
    pub vel: Vector2,
    sprite: (Texture2D, Texture2D),
    punch: bool,
    hit: (Color, f64),
    pub hp: f32,
    pub n: (i32, f64)
}

impl Man {
    pub fn new(pos: Vector2, vel: Vector2, sprite: (Texture2D, Texture2D)) -> Man {
        Man {
            pos,
            vel,
            sprite,
            punch: true,
            hit: (Color::WHITE, 0.0),
            hp: 1.0,
            n: (get_random_value(1, 10), 0.0)
        }
    }

    fn control(&mut self, d: &RaylibDrawHandle, subs: &mut SubSpawn) {
        let a = 9.0;
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.vel = -(self.pos - d.get_mouse_position()).normalized() * a;
            self.punch = !self.punch;
            for i in &mut subs.subs {
                let hit = (i.pos - self.pos).angle_to(Vector2::new(1.0, 0.0)) + ((PI/3.0) as f32) > (self.look(d) * (PI as f32/180.0)) && (i.pos - self.pos).angle_to(Vector2::new(1.0, 0.0)) - ((PI/3.0) as f32) < (self.look(d) * (PI as f32/180.0)) && (i.pos - self.pos).length() < 100.0;

                if hit {
                    i.hp -= 1.0/self.n.0 as f32;
                    i.hit = (Color::RED, d.get_time());
                    println!("{}", i.hp);
                }
            }

        }
        self.pos += self.vel;
        self.vel -= self.vel.scale_by(0.1);
    }

    fn look(&self, d: &RaylibDrawHandle) -> f32 {
        (self.pos - d.get_mouse_position()).angle_to(Vector2::new(1.0,0.0)) * (((180.0)/PI) as f32) - 180.0
    }

    pub fn update(mut self, d: &mut RaylibDrawHandle, subs: &mut SubSpawn) -> Man {
        self.control(d, subs);
        let rec = Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: 128.0,
            height: 128.0
        };
        let rec2 = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 128.0,
            height: (2*(self.punch as i32)-1) as f32 * 128.0
        };
        d.draw_texture_pro(&self.sprite.0, rec2, rec, Vector2::new(64.0, 64.0),self.look(&d), self.hit.0);
        d.draw_texture_pro(&self.sprite.1, rec2, rec, Vector2::new(64.0, 64.0),self.look(&d), self.hit.0);

        if d.get_time() > self.n.1 + 5.0 {
            self.n.1 = d.get_time();
            self.n.0 = get_random_value(1, 10);
        }

        if d.get_time() - self.hit.1 > 0.2 {
            self.hit.0 = Color::WHITE;
        }

        for i in &subs.subs {
            if (i.pos - self.pos).length() < 50.0 && get_random_value::<i32>(1, 40) == 5 {
                 self.hp -= 0.1;
                 self.hit = (Color::RED, d.get_time());
            }
        }

        println!("{:?}, {}", self.pos, self.n.0);

        self
    }

}

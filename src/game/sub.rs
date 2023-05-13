use raylib::prelude::*;
use raylib::misc::get_random_value;

pub struct Sub<'a> {
    pub pos: Vector2,
    vel: f32,
    sprite: &'a Texture2D,
    pub hit: (Color, f64),
    pub hp: f32
}

impl<'a> Sub<'a> {
    pub fn new(pos: Vector2, vel: f32, sprite: &Texture2D) -> Sub {
        Sub {
            pos,
            vel,
            sprite,
            hit: (Color::WHITE, 0.0),
            hp: 1.0
        }
    }

    fn walk(&mut self, man_pos: Vector2) {
        self.pos += (man_pos - self.pos) * self.vel;
    }

    fn look(&self, man_pos: Vector2) -> f32 {
        if self.pos.distance_to(man_pos) > 1.0 {
            (self.pos - man_pos).angle_to(Vector2::new(1.0,0.0)) * (((180 as f64)/PI) as f32) + 90.0
        } else {
            0.0
        }
    }

    pub fn update(&mut self, man_pos: Vector2, d: &mut RaylibDrawHandle) {
        self.walk(man_pos);
        let rec1 = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 128.0,
            height: 128.0
        };
        let rec2 = Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: 128.0,
            height: 128.0
        };
        if d.get_time() - self.hit.1 > 0.2 {
            self.hit.0 = Color::WHITE;
        }
        d.draw_texture_pro(&self.sprite, rec1, rec2, Vector2::new(64.0, 64.0), self.look(man_pos), self.hit.0);
    }
}

pub struct SubSpawn<'a> {
    pub subs: Vec<Sub<'a>>,
    sprite: &'a Texture2D
}

impl<'a> SubSpawn<'a> {
    pub fn new(sprite: &'a Texture2D) -> SubSpawn<'a> {
        SubSpawn { subs: vec![], sprite }
    }

    pub fn update(&mut self, man_pos: Vector2, t: f64,d: &mut RaylibDrawHandle, sizex: &i32, sizey: &i32) {
        if t as i32 % 4 == 0 && t - 0.01 < (t as i32) as f64 {
            let angle: i32 = get_random_value(0, 360);
            let angle = angle as f32 * 180.0/PI as f32;
            self.subs.push(Sub::new(Vector2::new((sizex/2) as f32 * angle.cos(), (sizey/2) as f32 * angle.sin()), 0.005, self.sprite));
        }
        for i in &mut self.subs {
            if i.hp < 0.0 {
                drop(i);
                continue
            }
            i.update(man_pos, d);
        }
    }
}

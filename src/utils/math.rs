use bevy::prelude::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum Val {
    Fix(i64),
    Float(i64, i64),
}

impl Val {
    pub fn to_i64(&self) -> i64 {
        match self {
            Val::Fix(val) => *val,
            Val::Float(min, max) => {
                let mut rng = thread_rng();
                rng.gen_range(*min..(*max + 1))
            },
        }
    }
}


pub fn cal_vector_by_two_points(src: Vec2, dst: Vec2) -> Vec2 {
    let x_direcion = dst.x - src.x;
    let y_direcion = dst.y - src.y;
    
    let bevel = (x_direcion * x_direcion + y_direcion * y_direcion).sqrt();
    if bevel == 0.0 {
        return Vec2 { x: 0.0, y: 1.0 }
    }

    Vec2 { x: x_direcion / bevel, y: y_direcion / bevel }
}



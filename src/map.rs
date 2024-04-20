use std::borrow::Borrow;

use crate::{
    app::{HEIGHT, WIDTH},
    creatures::{AbsoluteCoords, LivingCell},
};

pub struct MapCoords(pub i32, pub i32);

impl From<AbsoluteCoords> for MapCoords {
    fn from(value: AbsoluteCoords) -> Self {
        let t = (value.0 + WIDTH, value.1 + HEIGHT);
        Self(t.0 as i32, t.1 as i32)
    }
}

/// The Vector Map is responsible for the virtual translation of outside world and how the creature sees it
pub struct VectorMap {
    v: Vec<VectorMapStates>,
    width: i32,
    height: i32,
}

pub enum VectorMapStates {
    Void,

    /// Represents a Creature on the Map holding its Id
    Creature(usize, AbsoluteCoords),
}

impl VectorMap {
    pub fn new(rc: Vec<LivingCell>) -> Self {
        let mut v: Vec<VectorMapStates> = Vec::new();

        for _ in 0..((WIDTH as u32 * 2) * (HEIGHT as u32) * 2) {
            v.push(VectorMapStates::Void);
        }

        // Load all LivingCell Instances in
        for (i, c) in rc.iter().enumerate() {
            let pos = c.get_coords();
            let ii = xy_to_usize(pos.into(), WIDTH as i32);
            if let Some(v) = v.get_mut(ii) {
                *v = VectorMapStates::Creature(i, pos);
            }
        }

        Self {
            width: WIDTH as i32 * 2,
            height: HEIGHT as i32 * 2,
            v,
        }
    }

    pub fn get_xy(&self, c: MapCoords) -> Option<&VectorMapStates> {
        if (c.0 > 0 && c.0 < self.width) && (c.1 >= 0 && c.1 < self.height) {
            let i = xy_to_usize(c, self.width);
            self.v.get(i)
        } else {
            None
        }
    }
}

fn xy_to_usize(c: MapCoords, w: i32) -> usize {
    ((c.1 * w) + c.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testConversion() {
        {
            let ac = AbsoluteCoords(0.0, 0.0);
            let mc: MapCoords = ac.into();

            assert_eq!(mc.0, WIDTH as i32);
            assert_eq!(mc.1, HEIGHT as i32);
        }

        {
            let ac = AbsoluteCoords(-WIDTH, -HEIGHT);
            let mc: MapCoords = ac.into();

            assert_eq!(mc.0, 0);
            assert_eq!(mc.1, 0);
        }
    }
}

use seeded_random::Random;

use crate::{
    app::{ParentRef, HEIGHT, WIDTH},
    map::{MapCoords, VectorMap, VectorMapStates},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbsoluteCoords(pub f64, pub f64);

#[derive(Debug, Clone)]
pub struct LivingCell {
    position: AbsoluteCoords,
    state: CellState,
    parent: ParentRef,
    next_state: Actions,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
enum Actions {
    DoNothing,
    Look(Direction),
    Go(Direction),
}

#[derive(Debug, Clone)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    LeftForward,
    RightForward,
    LeftBackward,
    RightBackward,
    Idle,
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Forward,
            1 => Self::Backward,
            2 => Self::Left,
            3 => Self::Right,
            4 => Self::LeftForward,
            5 => Self::LeftBackward,
            6 => Self::RightForward,
            7 => Self::RightBackward,
            _ => Self::Idle,
        }
    }
}

impl Direction {
    fn random() -> Self {
        let rnd = Random::new();
        let c = rnd.range(0, 8);
        Self::from(c)
    }
    fn get_vector(&self) -> AbsoluteCoords {
        match self {
            // (x,y)
            Direction::Forward => AbsoluteCoords(0.0, 1.0),
            Direction::Backward => AbsoluteCoords(0.0, -1.0),
            Direction::Left => AbsoluteCoords(-1.0, 0.0),
            Direction::Right => AbsoluteCoords(1.0, 0.0),
            Direction::LeftForward => AbsoluteCoords(-1.0, 1.0),
            Direction::RightForward => AbsoluteCoords(1.0, 1.0),
            Direction::LeftBackward => AbsoluteCoords(-1.0, -1.0),
            Direction::RightBackward => AbsoluteCoords(1.0, -1.0),
            Direction::Idle => AbsoluteCoords(0.0, 0.0),
        }
    }
}

impl Actions {
    pub fn random() -> Self {
        let random = Random::new();
        let r = random.range(0, 4);
        match r {
            0 => Self::Go(Direction::Forward),
            1 => Self::Go(Direction::Backward),
            2 => Self::Go(Direction::Left),
            3 => Self::Go(Direction::Right),
            4 => Self::Go(Direction::LeftForward),
            5 => Self::Go(Direction::LeftBackward),
            6 => Self::Go(Direction::RightForward),
            7 => Self::Go(Direction::RightBackward),
            _ => Self::DoNothing,
        }
    }
}

impl LivingCell {
    pub fn new(parent: ParentRef) -> Self {
        let random = Random::new();
        random.range(0, 100);

        let random_x = random.range(0, 360) as f64;
        let random_y = random.range(0, 180) as f64;
        //let random_y = source.read_f64() * 100.0 - 50.0;

        //let (tx, rx) = mpsc::channel();
        let cell = Self {
            position: AbsoluteCoords(random_x - 180.0, random_y - 90.0),
            state: CellState::Alive,
            next_state: Actions::DoNothing,
            parent,
        };

        //let thread = Self::get_thread_handle(rx, parent_vector);

        cell
    }
    pub fn get_coords(&self) -> AbsoluteCoords {
        self.position
    }

    pub fn kill(&mut self) {
        self.state = CellState::Dead
    }

    fn safe_to_go(&self, context: &VectorMap, coordinates: AbsoluteCoords) -> bool {
        if let Some(e) = context.get_xy(coordinates.into()) {
            match e {
                VectorMapStates::Void => {
                    true
                },

                // We saw a creature there
                VectorMapStates::Creature(i, pos) => {
                    false
                },
            }
        } else {
            false
        }
    }

    pub fn tick(&mut self, context: &VectorMap) {
        if self.state != CellState::Dead {
            match &self.next_state {
                Actions::DoNothing => {
                    //Look around, gather information

                    self.next_state = Actions::random()
                }
                Actions::Look(d) => {
                    let d_vec = d.get_vector();
                    let new_pos = add_coords(&self.position, &d_vec);

                    if let Some(e) = context.get_xy(new_pos.into()) {
                        match e {
                            VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),

                            // We saw a creature there
                            VectorMapStates::Creature(i, pos) => {
                                println!("Creature ID {} at {:?}", i, pos);
                            }
                        }
                    }
                }
                Actions::Go(d) => {
                    let d_vec = d.get_vector();
                    let new_pos = add_coords(&self.position, &d_vec);
                    if self.safe_to_go(context, new_pos) {
                        self.position = new_pos
                    }
                }
            }
        }
    }
}

fn add_coords(a: &AbsoluteCoords, b: &AbsoluteCoords) -> AbsoluteCoords {
    AbsoluteCoords(a.0 + b.0, a.1 + b.1)
}

#[cfg(test)]
mod tests {
    use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn testMaps() {
        let cells = Rc::new(RefCell::from(Vec::new()));

        let c = LivingCell::new(cells);
        //cells.borrow_mut().push(c);
    }
}
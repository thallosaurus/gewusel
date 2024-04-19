use std::{
    cell::RefCell,
    rc::Rc,
};

use seeded_random::Random;

pub type Coords = (i32, i32);

/// The Vector Map is responsible for the virtual translation of outside world and how the creature sees it
pub struct VectorMap {
    v: Vec<VectorMapStates>,
    width: u32,
    height: u32,
}

enum VectorMapStates {
    Void,
    Creature(usize),
}

impl VectorMap {
    pub fn new(rc: Vec<LivingCell>) -> Self {
        let mut v: Vec<VectorMapStates> = Vec::new();

        for _ in 0..(360 * 180) {
            v.push(VectorMapStates::Void);
        }

        for (i, c) in rc.iter().enumerate() {
            let ii = xy_to_usize(c.position.0, c.position.1, 360);
            if let Some(v) = v.get_mut(ii) {
                *v = VectorMapStates::Creature(i);
            }
        }

        Self {
            width: 360,
            height: 180,
            v,
        }
    }

    fn get_xy(&self, x: i32, y: i32) -> Option<&VectorMapStates> {
        if (x < 0 || x > self.width as i32) || (y < 0 || y > self.height as i32) {
            return None
        }
        let i = xy_to_usize(x, y, self.width);
        self.v.get(i)
    }
}

fn xy_to_usize(x: i32, y: i32, w: u32) -> usize {
    let (x, y) = (x + 180, y + 90);
    ((y * w as i32) + x) as usize
}

#[derive(Debug, Clone)]
pub struct LivingCell {
    position: Coords,
    state: CellState,
    parent: Rc<RefCell<Vec<Self>>>,
    next_state: Actions, //thread_signal_sender: Sender<LivingCellSignals>,
}

#[derive(Debug, Clone)]
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
    RightBackward
}

impl Actions {
    pub fn random() -> Self {
        let random = Random::new();
        let r = random.range(0, 8);
        match r {
            0 => Self::Go(Direction::Forward),
            1 => Self::Go(Direction::Backward),
            2 => Self::Go(Direction::Left),
            3 => Self::Go(Direction::Right),
            _ => Self::DoNothing,
        }
    }
}

impl LivingCell {
    pub fn new(parent: Rc<RefCell<Vec<Self>>>) -> Self {
        let random = Random::new();
        random.range(0, 100);

        let random_x = random.range(0, 360) as i32;
        let random_y = random.range(0, 180) as i32;
        //let random_y = source.read_f64() * 100.0 - 50.0;

        //let (tx, rx) = mpsc::channel();
        let cell = Self {
            position: (random_x - 180, random_y - 90),
            state: CellState::Alive,
            next_state: Actions::DoNothing,
            parent,
        };

        //let thread = Self::get_thread_handle(rx, parent_vector);

        cell
    }
    pub fn get_coords(&self) -> Coords {
        self.position
    }

    pub fn kill(&self) {}

    pub fn tick(&mut self, context: &VectorMap) {
        match &self.next_state {
            Actions::DoNothing => {
                //Look around, gather information

                self.next_state = Actions::random()
            }
            Actions::Look(d) => {
                match d {
                    Direction::Forward => {
                        if let Some(e) = context.get_xy(self.position.0 + 1, self.position.1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    }
                    Direction::Backward => {
                        if let Some(e) = context.get_xy(self.position.0 - 1, self.position.1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    }
                    Direction::Left => {
                        if let Some(e) = context.get_xy(self.position.0, self.position.1 - 1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    }
                    Direction::Right => {
                        if let Some(e) = context.get_xy(self.position.0, self.position.1 + 1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    }
                    Direction::LeftForward => {
                        if let Some(e) = context.get_xy(self.position.0 + 1, self.position.1 - 1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    },
                    Direction::RightForward => {
                        if let Some(e) = context.get_xy(self.position.0 + 1, self.position.1 + 1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    },
                    Direction::LeftBackward => {
                        if let Some(e) = context.get_xy(self.position.0 - 1, self.position.1 - 1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    },
                    Direction::RightBackward => {
                        if let Some(e) = context.get_xy(self.position.0 + 1, self.position.1 + 1) {
                            match e {
                                VectorMapStates::Void => self.next_state = Actions::Go(d.clone()),
                                VectorMapStates::Creature(_) => {}
                            }
                        }
                    },
                }
            }
            Actions::Go(d) => match d {
                Direction::Forward => {
                    if self.position.0 < 180 {
                        self.position.0 += 1
                    }
                }
                Direction::Backward => {
                    if self.position.0 > -180 {
                        self.position.0 -= 1
                    }
                }
                Direction::Left => {
                    if self.position.1 > -90 {
                        self.position.1 -= 1
                    }
                }
                Direction::Right => {
                    if self.position.1 < 90 {
                        self.position.1 += 1
                    }
                }
                Direction::LeftForward => {
                    self.position.0 += 1;
                    self.position.1 -= 1;

                },
                Direction::RightForward => {
                    self.position.0 += 1;
                    self.position.1 += 1;
                },
                Direction::LeftBackward => {
                    self.position.0 -= 1;
                    self.position.1 -= 1;
                },
                Direction::RightBackward => {
                    self.position.0 += 1;
                    self.position.1 += 1;
                },
            },
        }
    }
}

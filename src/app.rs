use std::{cell::RefCell, io, rc::Rc, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{
        canvas::Canvas, Block, Borders, Widget
    },
    Frame,
};

use ratatui::{
    style::Color,
    widgets::canvas::*,
};

use crate::{creatures::LivingCell, map::VectorMap, tui};

pub const WIDTH: f64 = 180.0;
pub const HEIGHT: f64 = 90.0;

pub type ParentRef = Rc<RefCell<Vec<LivingCell>>>;


#[derive(Debug)]
pub struct App {
    //pub counter: u8,
    pub exit: bool,
    cells: ParentRef
}

impl Default for App {
    fn default() -> Self {
        let cells = Rc::new(RefCell::from(Vec::new()));
        
        for _ in 0..100 {
            cells.borrow_mut().push(LivingCell::new(cells.clone()))
        }

        Self { exit: Default::default(), cells }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {

        while !self.exit {
            // handle logic
            self.tick_cells();

            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Iterate through and tick each Cell
    fn tick_cells(&mut self) {
        let mut b = self.cells.borrow_mut();

        // Get contextualized Map for Cells
        let vm = VectorMap::new(b.to_owned());

        for c in b.iter_mut() {
            c.tick(&vm);
        }
    }

    /// Renders the frame
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// Handles keyboard events
    pub fn handle_events(&mut self) -> io::Result<()> {
        let tick_rate = Duration::from_millis(16);
        if event::poll(tick_rate)? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key)
            }
        };
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            //KeyCode::Left => self.decrement_counter(),
            //KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    /// Exits the program
    pub fn exit(&mut self) {
        for c in self.cells.borrow_mut().iter_mut() {
            c.kill();
        }
        self.exit = true;
    }
}

impl Widget for &App {

    /// Handles drawing of the dots
    fn render(self, area: Rect, buf: &mut Buffer) {
        let canvas = Canvas::default()
            .block(Block::default().title("Canvas").borders(Borders::ALL))
            .x_bounds([-1.0 * WIDTH, WIDTH])
            .y_bounds([-1.0 * HEIGHT, HEIGHT])
            .paint(|ctx| {
                let coords: Vec<(f64, f64)> = self.cells.borrow().iter().map(|c| {
                    let co = c.get_coords();

                    (co.0, co.1)
                }).collect();

                let shape = Points {
                    coords: &coords,
                    color: Color::Cyan
                };

                ctx.draw(&shape);
            });
        canvas.render(area, buf)
    }
}
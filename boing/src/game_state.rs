use ggez::event::EventHandler;
use ggez::{event::KeyCode, Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;

pub struct GameState {
    pub bats: [Bat; 2],
    pub ball: Ball,
    /// List of the current impacts to display.
    pub impacts: Vec<Impact>,
    /// Offset added to the AI player's target Y position, so it won't aim to hit the ball exactly in
    /// the centre of the bat.
    pub ai_offset: i8,
}

impl GameState {
    pub fn new(
        controls: (
            Option<Box<dyn Fn(KeyCode) -> i8>>,
            Option<Box<dyn Fn(KeyCode) -> i8>>,
        ),
    ) -> Self {
        Self {
            bats: [
                Bat {
                    player: 0,
                    move_func: controls.0,
                },
                Bat {
                    player: 1,
                    move_func: controls.1,
                },
            ],
            ball: Ball { dx: -1. },
            impacts: vec![],
            ai_offset: 0,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        todo!()
    }
}
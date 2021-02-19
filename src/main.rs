extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

extern crate rand;

use std::{collections::LinkedList, iter::FromIterator};

use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::PistonWindow;
use rand::{thread_rng, Rng};
use rust_snake::game::{
    food::Food,
    snake::{Direction, Snake, SnakePiece},
    Game,
};

fn main() {
    let opengl = OpenGL::V4_5;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let width = COLS * SQUARE_WIDTH;
    let height = ROWS * SQUARE_WIDTH;

    let mut window: PistonWindow = WindowSettings::new("Rust Snake", [width, height])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut r = thread_rng();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        food: Food {
            x: r.gen_range(0..COLS),
            y: r.gen_range(0..ROWS),
        },
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS / 4, ROWS / 2)]).into_iter()),
            width: SQUARE_WIDTH,
            d: Direction::RIGHT,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", game.score);
}

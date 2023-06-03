extern crate piston_window;
use piston_window::*;

mod snake;
use rand::Rng;
use snake::{Direction, Position, Snake};

const GRID_SIZE: u32 = 32;
const COLS: u32 = 20;
const ROWS: u32 = 20;

const WIDTH: f64 = (GRID_SIZE * COLS) as f64;
const HEIGHT: f64 = (GRID_SIZE * ROWS) as f64;

fn draw_grid(context: &Context, graphics: &mut G2d) {
    for col in 0..=COLS {
        for row in 0..=ROWS {
            line(
                [0.5; 4],
                1.0,
                [
                    col as f64 * GRID_SIZE as f64,
                    0.0,
                    col as f64 * GRID_SIZE as f64,
                    HEIGHT,
                ],
                context.transform,
                graphics,
            );
            line(
                [0.5; 4],
                1.0,
                [
                    0.0,
                    row as f64 * GRID_SIZE as f64,
                    WIDTH,
                    row as f64 * GRID_SIZE as f64,
                ],
                context.transform,
                graphics,
            );
        }
    }
}

fn draw_food(food: &Position, context: &Context, graphics: &mut G2d) {
    rectangle(
        [0.0, 1.0, 0.0, 1.0],
        [
            food.0 as f64 * GRID_SIZE as f64,
            food.1 as f64 * GRID_SIZE as f64,
            GRID_SIZE as f64,
            GRID_SIZE as f64,
        ],
        context.transform,
        graphics,
    );
}

fn generate_food() -> Position {
    Position(
        rand::thread_rng().gen_range(0..COLS) as u32,
        rand::thread_rng().gen_range(0..ROWS) as u32,
    )
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake AI", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(5);

    let mut food = generate_food();

    let mut snake = Snake::new();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.1; 4], graphics);
            draw_grid(&context, graphics);
            draw_food(&food, &context, graphics);

            if snake.update(&food) {
                food = generate_food();
            }
            snake.draw(&context, graphics);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => snake.set_direction(Direction(0, -1)),
                Key::A => snake.set_direction(Direction(-1, 0)),
                Key::S => snake.set_direction(Direction(0, 1)),
                Key::D => snake.set_direction(Direction(1, 0)),
                _ => {}
            }
        }

        if snake.is_dead() {
            window.set_should_close(true);
        }
    }
}

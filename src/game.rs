/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
// Standard libraries
use std::io::Stdout;
use std::time;

// 3rd party crates
use crossterm::event::KeyCode;
use tokio::sync::mpsc::UnboundedReceiver;

// My crates
use crate::food::Food;
use crate::position::Coordinates;
use crate::snake::Snake;
use crate::snake::SnakeDirection;

pub struct SnakeGame {
    screen_size: Coordinates,
    dir: SnakeDirection,
    rx: UnboundedReceiver<KeyCode>,
}

impl SnakeGame {
    pub fn new(
        screen_size: Coordinates,
        dir: SnakeDirection,
        rx: UnboundedReceiver<KeyCode>,
    ) -> Self {
        Self {
            screen_size,
            dir,
            rx,
        }
    }
    fn listen_for_key_press(&mut self) -> SnakeDirection {
        match self.rx.try_recv() {
            Ok(key) => {
                if key == KeyCode::Up && self.dir != SnakeDirection::Down {
                    SnakeDirection::Up
                } else if key == KeyCode::Down && self.dir != SnakeDirection::Up {
                    SnakeDirection::Down
                } else if key == KeyCode::Left && self.dir != SnakeDirection::Right {
                    SnakeDirection::Left
                } else if key == KeyCode::Right && self.dir != SnakeDirection::Left {
                    SnakeDirection::Right
                } else if key == KeyCode::Esc {
                    SnakeDirection::Esc
                } else {
                    self.dir
                }
            }
            Err(_e) => self.dir,
        }
    }
    pub fn run(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let mut snake = Snake::new();
        let mut food = Food::new();
        let delay = time::Duration::from_millis(30);

        food.init_food(self.screen_size);
        snake.init_snake(self.screen_size);
        food.create_food(&snake.snake_body);
        while snake.is_alive {
            self.dir = self.listen_for_key_press();
            if self.dir == SnakeDirection::Esc {
                break;
            }
            self.draw_snake(&mut snake, self.dir, stdout)?;
            self.draw_food(&mut food, stdout)?;

            if snake.head == food.food_position {
                snake.grow_snake(food.food_position);
                food.create_food(&snake.snake_body);
            }

            std::thread::sleep(delay);
        }
        Ok(())
    }

    fn draw_snake(
        &mut self,
        snake: &mut Snake,
        dir: SnakeDirection,
        stdout: &mut Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        snake.remove_trail(stdout)?;
        snake.set_direction(dir);
        snake.crawl_snake();
        snake.display_snake(stdout)?;
        Ok(())
    }

    fn draw_food(
        &mut self,
        food: &mut Food,
        stdout: &Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        food.display_food(stdout)?;
        Ok(())
    }
}
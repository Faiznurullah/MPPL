use wasm_bindgen::prelude::*;
use crate::game::snake::Snake;
use crate::game::food::Food;

#[wasm_bindgen]
pub struct Game {
    snake: Snake,
    food: Food,
    width: usize,
    height: usize,
    score: i32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let snake = Snake::new();
        let food = Food::new();
        Game {
            snake,
            food,
            width: 20,
            height: 20,
            score: 0,
        }
    }

    pub fn start(&mut self) {
        self.snake.reset();
        self.food.respawn();
        self.score = 0;
    }

    pub fn update(&mut self) {
        let should_grow = self.snake.check_food_collision(&self.food);
        
        self.snake.move_and_maybe_grow(should_grow);
        
        if should_grow {
            self.score += 1;
            self.food.respawn();
        }
        
        if self.snake.check_collision() || self.snake.check_wall_collision(self.width as i32, self.height as i32) {
            self.start();
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_snake_positions(&self) -> js_sys::Array {
        let positions = self.snake.get_body();
        let js_array = js_sys::Array::new();
        for pos in positions {
            let coord_array = js_sys::Array::new();
            coord_array.push(&wasm_bindgen::JsValue::from(pos.0));
            coord_array.push(&wasm_bindgen::JsValue::from(pos.1));
            js_array.push(&coord_array);
        }
        js_array
    }

    pub fn get_food_x(&self) -> i32 {
        self.food.get_position().0
    }

    pub fn get_food_y(&self) -> i32 {
        self.food.get_position().1
    }

    pub fn check_collision(&self) -> bool {
        self.snake.check_collision() || self.snake.check_wall_collision(self.width as i32, self.height as i32)
    }

    pub fn set_direction(&mut self, x: i32, y: i32) {
        self.snake.set_direction(x, y);
    }
}
mod game;

use wasm_bindgen::prelude::*;
use crate::game::Game;

#[wasm_bindgen]
pub struct SnakeGame {
    game: Game,
}

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SnakeGame {
        SnakeGame {
            game: Game::new(),
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.game.start();
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.game.update();
    }

    #[wasm_bindgen]
    pub fn get_snake_positions(&self) -> js_sys::Array {
        self.game.get_snake_positions()
    }

    #[wasm_bindgen]
    pub fn get_food_x(&self) -> i32 {
        self.game.get_food_x()
    }

    #[wasm_bindgen]
    pub fn get_food_y(&self) -> i32 {
        self.game.get_food_y()
    }

    #[wasm_bindgen]
    pub fn check_collision(&self) -> bool {
        self.game.check_collision()
    }

    #[wasm_bindgen]
    pub fn set_direction(&mut self, x: i32, y: i32) {
        self.game.set_direction(x, y);
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> i32 {
        self.game.get_score()
    }
}
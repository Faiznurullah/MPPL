use std::collections::VecDeque;
use crate::game::food::Food;

pub struct Snake {
    body: VecDeque<(i32, i32)>,
    direction: (i32, i32),
}

impl Snake {
    pub fn new() -> Snake {
        let mut body = VecDeque::new();
        body.push_back((5, 5)); 
        Snake {
            body,
            direction: (1, 0),
        }
    }

    pub fn reset(&mut self) {
        self.body.clear();
        self.body.push_back((5, 5));
        self.direction = (1, 0);
    }

    pub fn set_direction(&mut self, x: i32, y: i32) { 
        if (x != 0 && self.direction.0 != -x) || (y != 0 && self.direction.1 != -y) {
            self.direction = (x, y);
        }
    } 

    pub fn move_and_maybe_grow(&mut self, should_grow: bool) {
        let head = self.body.front().unwrap();
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.body.push_front(new_head);
        
        if !should_grow {
            self.body.pop_back();
        }
    }

    pub fn check_collision(&self) -> bool {
        let head = self.body.front().unwrap();
        // Check if the snake collides with itself
        for segment in self.body.iter().skip(1) {
            if head == segment {
                return true;
            }
        }
        false
    }

    pub fn check_wall_collision(&self, width: i32, height: i32) -> bool {
        let head = self.body.front().unwrap();
        head.0 < 0 || head.0 >= width || head.1 < 0 || head.1 >= height
    }

    pub fn check_food_collision(&self, food: &Food) -> bool {
        let head = self.body.front().unwrap();
        *head == food.get_position()
    }

    pub fn get_body(&self) -> Vec<(i32, i32)> {
        self.body.iter().cloned().collect()
    }
}
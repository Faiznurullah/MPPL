use rand::Rng;

pub struct Food {
    position: (i32, i32),
}

impl Food {
    pub fn new() -> Food {
        Food {
            position: Food::spawn_food(),
        }
    }

    fn spawn_food() -> (i32, i32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..20), rng.gen_range(0..20))
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    pub fn respawn(&mut self) {
        self.position = Food::spawn_food();
    }
}
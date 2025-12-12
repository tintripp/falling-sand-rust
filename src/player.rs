pub struct Player{
    pub x: i32
}

impl Player {
    pub fn update(&mut self) {
        self.x += 1;
    }
}
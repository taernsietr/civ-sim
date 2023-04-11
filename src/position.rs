use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Position { x: u32, y: u32 }

impl Position {
    fn get(self) -> (u32, u32) {
        (self.x, self.y)
    }
    
    fn set(&mut self, nx: u32, ny: u32) {
        self.x = nx;
        self.y = ny;
    }

    fn distance_to(self, other: &Self) -> f32 {
        /* sqrt of the sum of the squares of the differences */
        ((other.x - self.x).pow(2) as f32 + (other.y - self.y).pow(2) as f32).sqrt()
    }
}


use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        game.draw();
        if game.quit {
            return;
        }
        next_frame().await
    }
}

pub struct Game {
    pub quit: bool,
    pub player_state: PlayerState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            quit: false,
            player_state: PlayerState {
                position: Vec2::new(100f32, 100f32),
                rotation: 0f32,
            },
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.quit = true;
        }
    }

    pub fn draw(&self) {
        clear_background(color_u8!(0, 211, 205, 205));

        draw_poly_lines(
            self.player_state.position.x,
            self.player_state.position.y,
            3,
            10.,
            self.player_state.rotation * 180. / std::f32::consts::PI - 90.,
            2.,
            BLACK,
        );

        draw_box(Vec2::new(200f32, 200f32), Vec2::new(10f32, 10f32));
    }
}

pub struct PlayerState {
    pub position: Vec2,
    pub rotation: f32,
}

fn draw_box(pos: Vec2, size: Vec2) {
    let dimension = size * 2.;
    let upper_left = pos - size;

    draw_rectangle(upper_left.x, upper_left.y, dimension.x, dimension.y, BLACK);
}

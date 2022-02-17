use macroquad::prelude::*;
use serde_json;
use shared::ServerMessage;
use ws::Connection;

mod ws;

#[macroquad::main("game")]
async fn main() {
    let mut connection = Connection::new();
    connection.connect("ws://localhost:3030/game");

    let mut game = Game::new().await;

    loop {
        if let Some(msg) = connection.poll() {     
            if let ServerMessage::Welcome(welcome_msg) = 
                serde_json::from_slice::<ServerMessage>(msg.as_slice())
                    .expect("deserialization failed") {
                println!("Welcome {}", welcome_msg);
            }
        }

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
    pub texture: Texture2D,
}

impl Game {
    pub async fn new() -> Self {
        let texture = load_texture("assets/plane.png").await.unwrap();

        Self {
            quit: false,
            player_state: PlayerState {
                position: Vec2::new(100f32, 100f32),
                rotation: 0f32,
            },
            texture,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.quit = true;
        }

        const ROT_SPEED: f32 = 0.015;

        if is_key_down(KeyCode::Right) {
            self.player_state.rotation += ROT_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.player_state.rotation -= ROT_SPEED;
        }

        const SPEED: f32 = 0.6;

        self.player_state.position += vec2_from_angle(self.player_state.rotation) * SPEED;

        if self.player_state.position.x > screen_width() {
            self.player_state.position.x = -self.texture.width();
        } else if self.player_state.position.x < -self.texture.width() {
            self.player_state.position.x = screen_width();
        }

        if self.player_state.position.y > screen_height() {
            self.player_state.position.y = -self.texture.height();
        } else if self.player_state.position.y < -self.texture.height() {
            self.player_state.position.y = screen_height();
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

        draw_texture_ex(
            self.texture,
            self.player_state.position.x,
            self.player_state.position.y,
            WHITE,
            DrawTextureParams {
                rotation: self.player_state.rotation,
                ..Default::default()
            },
        );
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

pub fn vec2_from_angle(angle: f32) -> Vec2 {
    let angle = angle - std::f32::consts::FRAC_PI_2;
    Vec2::new(angle.cos(), angle.sin())
}

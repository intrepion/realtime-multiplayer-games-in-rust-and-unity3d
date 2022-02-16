use macroquad::prelude::*;

#[macroquad::main("game")]
async fn main() {
    loop {
        next_frame().await
    }
}

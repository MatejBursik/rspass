use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "RsPass".to_owned(),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    loop {
        clear_background(GRAY);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, BLACK);

        next_frame().await;
    }
}
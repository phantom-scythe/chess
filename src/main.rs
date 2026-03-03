use macroquad::{prelude::*, color};
use miniquad::conf::Icon;

#[macroquad::main(window_conf)]
async fn main() {
    let background_color = color::Color::from_rgba(240, 240, 240, 255);
    let sprite_color = color::Color::from_rgba(230, 230, 250, 255);
    let title_color = color::Color::from_rgba(54, 69, 79, 255);

    let board_black: Texture2D = load_texture("assets/chess_black_perspective_board.png")
        .await
        .unwrap();
    let board_white: Texture2D = load_texture("assets/chess_white_perspective_board.png")
        .await
        .unwrap();

    let white_press: Texture2D = load_texture("assets/white_press.png").await.unwrap();
    let white_hover: Texture2D = load_texture("assets/white_hover.png").await.unwrap();
    let black_press: Texture2D = load_texture("assets/black_press.png").await.unwrap();
    let black_hover: Texture2D = load_texture("assets/black_hover.png").await.unwrap();

    let mut player_color = "white";

    let mut state = State::MENU;

    let title = "Which color do you want to play as ?";
    let font_size = 32.;
    let title_size = measure_text(title, None, font_size as u16, 1.0);



    loop {
        clear_background(background_color);
        let screen_width = screen_width();
        let screen_height = screen_height();

        match state {
            State::PLAY => {
                // Here it is considered that if in case you modify the code, the dimension of both board is same.
                let board_position = (
                    (screen_width - board_black.width()) / 2.,
                    (screen_height - board_white.height()) / 2.,
                );
                match player_color {
                    "black" => {
                        draw_texture(
                            &board_black,
                            board_position.0,
                            board_position.1,
                            sprite_color,
                        );
                    }
                    "white" => {
                        draw_texture(
                            &board_white,
                            board_position.0,
                            board_position.1,
                            sprite_color,
                        );
                    }
                    _ => {}
                }
            }
            State::MENU => {
                draw_text(
                    title,
                    (screen_width - title_size.width) / 2.,
                    screen_height / 6.,
                    font_size,
                    title_color,
                );
                let mouse_pos = mouse_position();
                if mouse_pos.0 >= (screen_width - black_hover.width()) * 1. / 6.
                    && mouse_pos.0
                        <= (screen_width - black_hover.width()) * 1. / 6. + black_hover.width()
                    && mouse_pos.1 >= screen_height * 2. / 6.
                    && mouse_pos.1 <= screen_height * 2. / 6. + black_hover.height()
                {
                    draw_texture(
                        &black_press,
                        (screen_width - black_hover.width()) * 1. / 6.,
                        screen_height * 2. / 6.,
                        background_color,
                    );
                    if is_mouse_button_pressed(MouseButton::Left) {
                        state = State::PLAY;
                        player_color = "black";
                        request_new_screen_size(board_black.width(), board_black.height());
                    }
                } else {
                    draw_texture(
                        &black_hover,
                        (screen_width - black_hover.width()) * 1. / 6.,
                        screen_height * 2. / 6.,
                        background_color,
                    );
                }

                if mouse_pos.0 >= (screen_width - white_hover.width()) * 5. / 6.
                    && mouse_pos.0
                        <= (screen_width - white_hover.width()) * 5. / 6. + white_hover.width()
                    && mouse_pos.1 >= screen_height * 2. / 6.
                    && mouse_pos.1 <= screen_height * 2. / 6. + white_hover.height()
                {
                    draw_texture(
                        &white_press,
                        (screen_width - white_hover.width()) * 5. / 6.,
                        screen_height * 2. / 6.,
                        background_color,
                    );
                    if is_mouse_button_pressed(MouseButton::Left) {
                        state = State::PLAY;
                        player_color = "white";
                        request_new_screen_size(board_white.width(), board_white.height());
                    }
                } else {
                    draw_texture(
                        &white_hover,
                        (screen_width - white_hover.width()) * 5. / 6.,
                        screen_height * 2. / 6.,
                        background_color,
                    );
                }
            }
            State::REPLAY => {}
        }
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_width: 506,
        window_height: 220,
        window_resizable: true,
        icon: Some(set_icon()),
        ..Default::default()
    }
}

const fn set_icon() -> Icon {
    let icon16: [u8; 1024] = *include_bytes!("icon16.rgba");
    let icon32: [u8; 4096] = *include_bytes!("icon32.rgba");
    let icon64: [u8; 16384] = *include_bytes!("icon64.rgba");
    Icon {
        small: icon16,
        medium: icon32,
        big: icon64,
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum State {
    PLAY,
    MENU,
    REPLAY,
}

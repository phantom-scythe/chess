use macroquad::{color, prelude::*};
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

    // FEN String and i removed the additional content and will monitor it separately.
    let mut fen: String = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();

    let chess_piece: Texture2D = load_texture("assets/chess_pieces_sprite.png")
        .await
        .unwrap();

    let mut reverse_fen = 1;

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

                        // just to remind you when we change the perspective just reverse the string
                        if reverse_fen == 1 {
                            fen = fen.chars().rev().collect();
                            reverse_fen -= 1;
                        }
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

                let mut piece_position = board_position;
                for x in 0..fen.len() {
                    if fen.chars().nth(x).unwrap().is_ascii_digit() {
                        if let Some(digit) = fen.chars().nth(x).unwrap().to_digit(10) {
                            piece_position.0 += 64. * digit as f32;
                        };
                    } else {
                        match fen.chars().nth(x).unwrap() {
                            // Here it is assumed that only the original svg is used where each piece is 64px by 64px and
                            // various other details, if you change the image make sure to change the code here.
                            'p' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 320.,
                                            y: 64.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'k' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 0.,
                                            y: 64.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'q' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 64.,
                                            y: 64.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'n' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 192.,
                                            y: 64.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'r' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 256.,
                                            y: 64.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'b' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 128.,
                                            y: 64.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'P' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 320.,
                                            y: 0.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'K' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 0.,
                                            y: 0.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'Q' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 64.,
                                            y: 0.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'N' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 192.,
                                            y: 0.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'R' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 256.,
                                            y: 0.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            'B' => {
                                draw_texture_ex(
                                    &chess_piece,
                                    piece_position.0,
                                    piece_position.1,
                                    sprite_color,
                                    DrawTextureParams {
                                        source: Some(Rect {
                                            x: 128.,
                                            y: 0.,
                                            w: 64.,
                                            h: 64.,
                                        }),
                                        ..Default::default()
                                    },
                                );
                                piece_position.0 += 64.;
                            }
                            '/' => {
                                piece_position.0 = board_position.0;
                                piece_position.1 += 64.;
                            }
                            _ => {
                                println!("Invalid FEN String");
                            }
                        }
                    }
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
        }
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_width: 560,
        window_height: 560,
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

#[derive(PartialEq, Eq)]
enum State {
    PLAY,
    MENU,
}

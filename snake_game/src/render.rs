use macroquad::prelude::*;
use crate::game::game_logic::Game;

pub fn draw_game(game: &Game) {

    let obs = game.observe();

    if obs.game_over {
        clear_background(WHITE);
        let text = "Game Over!";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
         );
    } else {
        clear_background(LIGHTGRAY);
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / obs.squares as f32;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        for i in 1..obs.squares {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..obs.squares {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        draw_rectangle(
            offset_x + obs.snake_head.0 as f32 * sq_size,
            offset_y + obs.snake_head.1 as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );

        for (x, y) in &obs.snake_body {
            draw_rectangle(
                offset_x + *x as f32 * sq_size,
                offset_y + *y as f32 * sq_size,
                sq_size,
                sq_size,
                LIME,
            );
        }

        draw_rectangle(
            offset_x + obs.fruit.0 as f32 * sq_size,
            offset_y + obs.fruit.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );

        draw_text(format!("SCORE: {}", obs.score).as_str(), 10., 20., 20., DARKGRAY);
    }
}
    
pub fn draw_game_in_rect(game: &Game, area: Rect, agent_name: &str) {

    let obs = game.observe();
   
    let margin = 60.;
    let game_size = (area.h - margin).min(area.w-margin);
    let sq_size = game_size / obs.squares as f32;
    let half_game = game_size/2.;

    let center_x = area.x + area.w/2.;
    let start_x = center_x - half_game;

    let center_y = area.y + area.h/2.;
    let start_y = center_y - half_game;

    let board = Rect::new(start_x, start_y, game_size, game_size);
    
           
    draw_rectangle(board.x, board.y, board.w, board.h, WHITE);
    //Horizontal lines
    for i in 1..obs.squares {
        draw_line(
            board.left(),
            board.y + sq_size * i as f32,
            board.right(),
            board.y + sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }
    //Vertical lines
    for i in 1..obs.squares {
        draw_line(
            board.left() + sq_size * i as f32,
            board.top(),
            board.left() + sq_size * i as f32,
            board.bottom(),
            2.,
            LIGHTGRAY,
        );
    }
    for (x, y) in &obs.snake_body {
        draw_rectangle(
         board.x + *x as f32 * sq_size,
         board.y + *y as f32 * sq_size,
            sq_size,
            sq_size,
            LIME,
        );
    }
    draw_rectangle(
        board.x + obs.snake_head.0 as f32 * sq_size,
        board.y + obs.snake_head.1 as f32 * sq_size,
        sq_size,
        sq_size,
        DARKGREEN,
    );
    draw_rectangle(
        board.x + obs.fruit.0 as f32 * sq_size,
        board.y + obs.fruit.1 as f32 * sq_size,
        sq_size,
        sq_size,
        GOLD,
    );
    let font_size = 0.09 * game_size;
    draw_text(format!("AGENT:{}", agent_name).as_str(), board.x, board.y - font_size*0.3, font_size, DARKGRAY);
    draw_text(format!(" SCORE:{}", obs.score).as_str(), board.x + font_size*7., board.y - font_size*0.3, font_size, DARKGRAY);

    if obs.game_over {
        let text = "Game Over!";
        let font_size = 0.1 * game_size;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            area.x + area.w / 2. - text_size.width / 2.,
            area.y + area.h / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

    } 
 
}

 
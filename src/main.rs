extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Rect;
use std::time::Duration;

// Game Variables
const ARRAY_SIZE: (usize, usize) = (320, 240);
const ARRAY_WINDOW_SCALE: usize = 4;

#[derive(Copy, Clone, std::fmt::Debug)]
pub enum Element {
    Air,
    Sand,
    Water,
    Stone
}

pub fn draw_square_terrain(
    arr: &mut [[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1], 
    elmnt: Element, 
    size: usize, 
    row: usize, 
    col: usize
) {
    for brush_iy in 0..size{
        for brush_ix in 0..size{
            let x = (col + brush_ix).saturating_sub(size / 2) as usize;
            let y = (row + brush_iy).saturating_sub(size / 2) as usize;

            //fit to arr size to prevent out of range errs
            if x < ARRAY_SIZE.0 && y < ARRAY_SIZE.1 {
                arr[y][x] = elmnt;
            }
        }
    }
}

pub fn draw_square_terrain_line(
    arr: &mut [[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1], 
    elmnt: Element, 
    size: usize,
    mut x0: i32, mut y0: i32, 
    x1: i32, y1: i32
) {

    let dx = (x1 - x0).abs();
    let mut sx = -1; 
    if x0 < x1{
        sx=1;
    }

    let dy = -(y1 - y0).abs();
    let mut sy = -1;
    if y0 < y1 {
        sy = 1;
    }

    let mut error = dx + dy;
    loop {
        draw_square_terrain(
            arr, 
            elmnt, 
            size as usize, 
            y0.max(0).min(ARRAY_SIZE.1 as i32) as usize, 
            x0.max(0).min(ARRAY_SIZE.0 as i32) as usize
        );
        
        let e2 = 2 * error;
        if e2 >= dy {
            if x0 == x1 {break;}
            error = error + dy;
            x0 = x0 + sx;
        }
        if e2 <= dx {
            if y0 == y1 {break;}
            error = error + dx;
            y0 = y0 + sy;
        }
    }
}

pub fn main() {
    // SDL setup
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        "rust-sdl3 demo", 
        (ARRAY_SIZE.0 * ARRAY_WINDOW_SCALE) as u32, 
        (ARRAY_SIZE.1 * ARRAY_WINDOW_SCALE) as u32
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Game Vars
    let mut matrix: [[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1] = 
        [[Element::Air; ARRAY_SIZE.0]; ARRAY_SIZE.1];

    let mut element_to_draw = Element::Sand;
    let mut brush_size = 1;

    let (mut old_mx, mut old_my) = (0, 0);
    let (mut mx, mut my);

    'running: loop {
        canvas.set_draw_color(Color::RGB(64,64,64));
        canvas.clear();

        // Event Loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                Event::MouseWheel { y, .. } => {
                    if brush_size as f32 + y > 0f32 {
                        brush_size = (brush_size as f32 + y) as usize;
                    }
                },

                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => matrix = 
                    [[Element::Air; ARRAY_SIZE.0]; ARRAY_SIZE.1],

                Event::KeyDown { keycode: Some(Keycode::_1), .. } => element_to_draw = Element::Sand,
                Event::KeyDown { keycode: Some(Keycode::_2), .. } => element_to_draw = Element::Water,
                Event::KeyDown { keycode: Some(Keycode::_3), .. } => element_to_draw = Element::Stone,
                Event::KeyDown { keycode: Some(Keycode::_0), .. } => element_to_draw = Element::Air,

                _ => {}
            }
        }

        // Update Loop
        let mouse_state = event_pump.mouse_state();
        if mouse_state.left() || mouse_state.right() {
            mx = (mouse_state.x() / (ARRAY_WINDOW_SCALE as f32)) as i32;
            my = (mouse_state.y() / (ARRAY_WINDOW_SCALE as f32)) as i32;

            let mut elmnt = element_to_draw;
            if mouse_state.right() {
                elmnt = Element::Air;
            }

            if old_mx == -1 || old_my == -1 {
                /*draw_square_terrain(
                    &mut matrix, elmnt, brush_size, 
                    old_mx, old_my
                );*/
            }else{
                draw_square_terrain_line(
                    &mut matrix, elmnt, brush_size, 
                    old_mx, old_my, 
                    mx, my
                );
            }

            old_mx = mx;
            old_my = my;

            //println!("Left click at col {} row {}!", m_col, m_row);
        }else {
            old_mx = -1;
            old_my = -1;
        }
        

        //reverse because if not we'd update the same sand all the way till it hits the ground
        for row_i in (0..matrix.len()-1).rev() {
            for col_i in 0..matrix[row_i].len() {
                let cell = matrix[row_i][col_i];

                if matches!(cell, Element::Sand){
                    //Try go down
                    if row_i+1 > matrix.len(){
                        continue;
                    }
                    
                    if matches!(matrix[row_i+1][col_i], Element::Air) {
                        let below = matrix[row_i+1][col_i];
                        //Can go down
                        //Swap elements
                        matrix[row_i][col_i] = below;
                        matrix[row_i+1][col_i] = cell;
                    } else {
                        //Can't go down! Panic!! 
                        //(Really, just check if bottom left then right is available. If so, swap to there!)
                        
                        //bottom left
                        if col_i > 0 && matches!(matrix[row_i+1][col_i-1], Element::Air) {
                            let bottom_left = matrix[row_i+1][col_i-1];
                            matrix[row_i][col_i] = bottom_left;
                            matrix[row_i+1][col_i-1] = cell;
                        }
                        
                        //bottom right
                        if col_i + 1 < ARRAY_SIZE.0 && matches!(matrix[row_i+1][col_i+1], Element::Air) {
                            let bottom_right = matrix[row_i+1][col_i+1];
                            matrix[row_i][col_i] = bottom_right;
                            matrix[row_i+1][col_i+1] = cell;
                        }
                    }
                }

                if matches!(cell, Element::Water){
                    //Try go down
                    if row_i+1 > matrix.len(){
                        continue;
                    }
                    
                    if matches!(matrix[row_i+1][col_i], Element::Air) {
                        let below = matrix[row_i+1][col_i];
                        matrix[row_i][col_i] = below;
                        matrix[row_i+1][col_i] = cell;
                    } else {
                        //Water: Can go left or right?
                        //left
                        if col_i > 0 && matches!(matrix[row_i][col_i-1], Element::Air) {
                            let left = matrix[row_i][col_i-1];
                            matrix[row_i][col_i] = left;
                            matrix[row_i][col_i-1] = cell;
                        }
                        
                        //right
                        if col_i + 1 < ARRAY_SIZE.0 && matches!(matrix[row_i][col_i+1], Element::Air) {
                            let right = matrix[row_i][col_i+1];
                            matrix[row_i][col_i] = right;
                            matrix[row_i][col_i] = cell;
                        }
                    }
                }
            }   
        }


        // Draw Loop
        for (row_i, row) in matrix.iter().enumerate() {
            for (col_i, col) in row.iter().enumerate() {
                canvas.set_draw_color(Color::RGB(64,64,64));
                if matches!(*col, Element::Sand){
                    canvas.set_draw_color(Color::RGB(255,255,64));
                }
                if matches!(*col, Element::Water){
                    canvas.set_draw_color(Color::RGB(64,128,255));
                }
                if matches!(*col, Element::Stone){
                    canvas.set_draw_color(Color::RGB(128,128,128));
                }
                canvas.fill_rect(
                    Rect::new(
                        (col_i * ARRAY_WINDOW_SCALE) as i32, 
                        (row_i * ARRAY_WINDOW_SCALE) as i32, 
                        ARRAY_WINDOW_SCALE as u32, 
                        ARRAY_WINDOW_SCALE as u32
                    )
                ).unwrap();
            }   
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }
}
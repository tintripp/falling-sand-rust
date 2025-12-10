extern crate sdl3;

use sdl3::mouse::MouseButton;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Rect;
use std::time::Duration;

pub fn main() {

    // Game Variables
    const ARRAY_SIZE: (usize, usize) = (320, 240);
    let array_window_scale = 4;

    #[derive(Copy, Clone, std::fmt::Debug)]
    enum Element {
        Air,
        Sand,
        Water
    }

    let mut matrix: [[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1] = 
        [[Element::Air; ARRAY_SIZE.0]; ARRAY_SIZE.1];
    matrix[0][0] = Element::Sand;
    matrix[0][1] = Element::Water;

    // SDL setup
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        "rust-sdl3 demo", 
        (ARRAY_SIZE.0 * array_window_scale) as u32, 
        (ARRAY_SIZE.1 * array_window_scale) as u32
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(64,64,64));
        canvas.clear();

        // Event Loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    println!("Space was PRESSED!! Spawning new sand at 0,0!");
                    matrix[0][0] = Element::Sand;
                },

                _ => {}
            }
        }

        // Update Loop
        let mouse_state = event_pump.mouse_state();
        if mouse_state.left() {
            let m_col = (mouse_state.x() as usize) / array_window_scale;
            let m_row = (mouse_state.y() as usize) / array_window_scale;

            matrix[m_row][m_col] = Element::Sand;

            println!("Left click at col {} row {}!", m_col, m_row);
        }


        //println!("{:?}", matrix); // Output: [[1, 0, 0], [0, 0, 5]]
        for row_i in 0..matrix.len() - 1 {
            for col_i in 0..matrix[row_i].len() {
                let cell = matrix[row_i][col_i];

                if matches!(cell, Element::Sand){
                    //if col_i - 1 >= 0 && matrix[col_i - 1]

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
                        //(Really, just check if bottom left then right is available. If so, go there!)
                        
                        //bottom left
                        if col_i > 0 && matches!(matrix[row_i+1][col_i-1], Element::Air) {
                            let bottom_left = matrix[row_i+1][col_i-1];
                            //Can go down
                            //Swap elements
                            matrix[row_i][col_i] = bottom_left;
                            matrix[row_i+1][col_i-1] = cell;

                            println!("Left diag land");
                        }
                        
                        //bottom right
                        if col_i + 1 < ARRAY_SIZE.0 && matches!(matrix[row_i+1][col_i+1], Element::Air) {
                            let bottom_right = matrix[row_i+1][col_i+1];
                            //Can go down
                            //Swap elements
                            matrix[row_i][col_i] = bottom_right;
                            matrix[row_i+1][col_i+1] = cell;

                            println!("Right diag land");
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
                canvas.fill_rect(
                    Rect::new(
                        (col_i * array_window_scale) as i32, 
                        (row_i * array_window_scale) as i32, 
                        array_window_scale as u32, 
                        array_window_scale as u32
                    )
                ).unwrap();
            }   
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }
}
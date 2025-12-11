extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::FRect;
use std::time::Duration;

// Game Variables
const FRAMERATE: u32 = 0;

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

    let texture_creator = canvas.texture_creator();
    let mut matrix_texture = texture_creator
        .create_texture_streaming(
            sdl3::pixels::PixelFormat::RGB24,
            ARRAY_SIZE.0 as u32,
            ARRAY_SIZE.1 as u32
        )
        .unwrap();
    matrix_texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);

    // A block of memory WE hold onto, bypassing the communication we'd have to do with the gpu using .with_lock()
    let mut matrix_texture_buffer = vec![0u8; ARRAY_SIZE.0 * ARRAY_SIZE.1 * 3];


    let mut element_to_draw = Element::Sand;
    let mut brush_size = 1;

    let (mut old_mx, mut old_my) = (0, 0);
    let (mut mx, mut my);    
    
    // Delta Time
    let performance_frequency = sdl3::timer::performance_frequency();
    let mut last_counter_value = sdl3::timer::performance_counter();

    'running: loop {
        let frame_start = std::time::Instant::now();

        let current_counter_value = sdl3::timer::performance_counter();
        let delta_ticks = current_counter_value - last_counter_value;
        
        let delta_time = delta_ticks as f64 / performance_frequency as f64;

        last_counter_value = current_counter_value;

        println!("FPS: {}", 1.0 / delta_time);

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


        ////////
        // TEXTURE CODE
        // Calling fill_rect EVERY time for EVERY pixel is REALLY expensive
        for y in 0..ARRAY_SIZE.1 {
            for x in 0..ARRAY_SIZE.0 {
                let offset = (y * ARRAY_SIZE.0 + x) * 3;

                let (r,g,b) = match matrix[y][x] {
                    Element::Sand => (255,255,64),
                    Element::Water => (64,128,255),
                    Element::Stone => (128,128,128),
                    _ => (64,64,64)
                };

                matrix_texture_buffer[offset] = r;
                matrix_texture_buffer[offset+1] = g;
                matrix_texture_buffer[offset+2] = b;
            }
        }

        matrix_texture.update(
            None, 
            &matrix_texture_buffer, 
            ARRAY_SIZE.0*3
        ).unwrap();


        canvas.copy(
            &matrix_texture,
            None,
            Some(
                FRect::new(
                    0f32, 
                    0f32, 
                    canvas.window().size().0 as f32, 
                    canvas.window().size().1 as f32
                )
            )
        ).unwrap();


        canvas.present();
        
        if FRAMERATE > 0{
            let target_frame_time = Duration::new(0, 1_000_000_000u32 / FRAMERATE);
            let frame_duration = frame_start.elapsed();
            if frame_duration < target_frame_time {
                std::thread::sleep(target_frame_time - frame_duration);
            }
        }
    }
}
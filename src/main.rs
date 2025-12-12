extern crate sdl3;
mod player;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::FRect;
use std::time::Duration;
use rand::Rng;
use player::Player;

// Game Variables
const FRAMERATE: u32 = 60;

const ARRAY_SIZE: (usize, usize) = (480, 360);
const ARRAY_WINDOW_SCALE: usize = 3;

#[derive(Copy, Clone, std::fmt::Debug, PartialEq, Eq)]
pub enum Element {
    Air,
    Sand,
    Water,
    Lava,
    Stone,
    WinArea,
    Grass
}

pub fn move_cell(
    arr: &mut [[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1], 
    col: usize, row: usize,
    move_x: i32, move_y: i32
) {
    let move_row = ((row as i32)+move_y) as usize;
    let move_col = ((col as i32)+move_x) as usize;

    let temp = arr[move_row][move_col];
    arr[move_row][move_col] = arr[row][col];
    arr[row][col] = temp;
}

pub fn cell_matches (
    arr: &[[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1], 
    col: usize, row: usize,
    elmnts: &[Element]
) -> bool {
    for elmnt in elmnts {
        if (col as i32) >= 0 && col < ARRAY_SIZE.0
        && (row as i32) >= 0 && row < ARRAY_SIZE.1 
        && arr[row][col] == *elmnt{
            return true;
        }
    }
    return false;

}

pub fn is_cell_empty (
    arr: &[[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1], 
    col: usize, row: usize
) -> bool {
    cell_matches(arr, col, row, &[Element::Air])

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
        "sand falling", 
        (ARRAY_SIZE.0 * ARRAY_WINDOW_SCALE) as u32, 
        (ARRAY_SIZE.1 * ARRAY_WINDOW_SCALE) as u32
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Game Vars
    let mut matrix: [[Element; ARRAY_SIZE.0]; ARRAY_SIZE.1] = 
        [[Element::Air; ARRAY_SIZE.0]; ARRAY_SIZE.1];

    let mut slim = Player { x: 5 };

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

    let mut rng = rand::rng();

    let lava_tick_speed = 3;
    let sand_sink_tick_speed = 20;

    'running: loop {
        let frame_start = std::time::Instant::now();

        let current_counter_value = sdl3::timer::performance_counter();
        let delta_ticks = current_counter_value - last_counter_value;
        
        let delta_time = delta_ticks as f64 / performance_frequency as f64;
        println!("FPS: {}", 1.0 / delta_time);

        last_counter_value = current_counter_value;


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
                Event::KeyDown { keycode: Some(Keycode::_4), .. } => element_to_draw = Element::Lava,
                Event::KeyDown { keycode: Some(Keycode::_5), .. } => element_to_draw = Element::Grass,
                Event::KeyDown { keycode: Some(Keycode::_6), .. } => element_to_draw = Element::WinArea,
                Event::KeyDown { keycode: Some(Keycode::_0), .. } => element_to_draw = Element::Air,


                Event::KeyDown { keycode: Some(Keycode::Space), .. } => draw_square_terrain(&mut matrix, Element::Water, 1, 0, 0),

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

            if old_mx != -1 && old_my != -1 {
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
                    
                    if cell_matches(&matrix, col_i, row_i + 1, &[Element::Air, Element::Water, Element::Lava]) {
                        if cell_matches(&matrix, col_i, row_i + 1, &[Element::Water, Element::Lava]){

                            if sdl3::timer::ticks() % sand_sink_tick_speed>0{
                                continue;
                            }

                        }
                        
                        move_cell(&mut matrix, col_i, row_i, 0, 1);
                    } else {
                        //check if bottom left then right is available. If so, swap to there!
                        //We must be able to go left/right before swapping with bottomleft/right.
                        
                        //bottom left
                        if col_i > 0 
                            && is_cell_empty(&matrix, col_i - 1, row_i)
                            && is_cell_empty(&matrix, col_i - 1, row_i + 1)
                        {
                            move_cell(&mut matrix, col_i, row_i, -1, 1);
                        }else
                        
                        //bottom right
                        if col_i + 1 < ARRAY_SIZE.0 
                            && is_cell_empty(&matrix, col_i + 1, row_i)
                            && is_cell_empty(&matrix, col_i + 1, row_i + 1)
                        {
                            move_cell(&mut matrix, col_i, row_i, 1, 1);
                        }
                    }
                }

                if matches!(cell, Element::Water) || matches!(cell, Element::Lava){
                    // TODO: liquids seem "biased", and this is ENTIRELY because
                    //         currently the array is writing as it reads through.

                    if matches!(cell, Element::Lava) && sdl3::timer::ticks() % lava_tick_speed>0{
                        continue;
                    }

                    //Try go down
                    if row_i+1 > matrix.len(){
                        continue;
                    }
                    
                    if is_cell_empty(&matrix, col_i, row_i + 1) {
                        move_cell(&mut matrix, col_i, row_i, 0, 1);
                    } 
                    
                    else {
                        //pick randomly either left or right to move
                        let direction: i32 = (rng.random_range(0..=1) * 2) - 1;

                        if (col_i as i32) >= 0 && col_i < ARRAY_SIZE.0  {
                            if is_cell_empty(&matrix, ((col_i as i32) + direction) as usize, row_i) {
                                move_cell(&mut matrix, col_i, row_i, direction, 0);
                            }
                        }
                    }

                    if matches!(cell, Element::Water){

                        for check_around_r in 0..=1{
                            let check_around_r: i32 = check_around_r * 2 - 1;
                            for check_around_c in 0..=1{
                                let check_around_c: i32 = check_around_c * 2 - 1;

                                let check_around_r = ((row_i as i32)+check_around_r) as usize;
                                let check_around_c = ((col_i as i32)+check_around_c) as usize;

                                if (check_around_r as i32) < 0 || check_around_r >= ARRAY_SIZE.1 
                                || (check_around_c as i32) < 0 || check_around_c >= ARRAY_SIZE.0{
                                    continue;
                                }

                                if matches!(matrix[check_around_r][check_around_c], Element::Lava) {
                                    matrix[row_i][col_i] = Element::Stone;
                                } 
                            }
                        
                        }


                    }

                }
            }   
        }


        // Update Player
        slim.update();



        // Draw Loop


        ////////
        // TEXTURE CODE
        // Calling fill_rect EVERY time for EVERY pixel is REALLY expensive
        for y in 0..ARRAY_SIZE.1 {
            for x in 0..ARRAY_SIZE.0 {
                let offset = (y * ARRAY_SIZE.0 + x) * 3;

                let (r,g,b) = match matrix[y][x] {
                    Element::Sand => (159,83,0),
                    Element::WinArea => (255,255,0),
                    Element::Water => (0,0,255),
                    Element::Lava => (255,0,0),
                    Element::Grass => (0,255,85),
                    Element::Stone => (0,0,0),
                    _ => (102,102,102)
                };

                matrix_texture_buffer[offset] = r;
                matrix_texture_buffer[offset+1] = g;
                matrix_texture_buffer[offset+2] = b;
            }
        }

        matrix_texture.update(
            None, 
            &matrix_texture_buffer, 
            ARRAY_SIZE.0*3 // 3 bytes for R, G, B
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
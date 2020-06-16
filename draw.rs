use piston_window::{rectangle, Context, G2d}; //{shape exists/imports the shape, draws 2d things(struct), buffers the graphics/shows the context on
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;                //suppose that this is a single pixel or block for the game or its walls. Can be any block too.
 
pub fn to_coord(game_coord: i32) -> f64 {    // scales the blocks to x and y co-ordinates. 20.0*x, 20.0*y = A large block spreads according to x and y co-ordinates.
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {    
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {  // for reference and understanding check page alpha in notes.
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], con.transform, g);
}
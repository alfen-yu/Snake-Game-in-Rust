use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;
 
const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.0];                   // snake color

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {                                                // Directions of the snake
    Up, 
    Down,
    Left,
    Right,
}

impl Direction {                                                    // to avoid using the opposite keys of the current direction otherwise the snake would bump in itself
    pub fn opposite(&self) -> Direction {
        match *self { 
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {                                                        //structure of a snake
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}   

impl Snake {                                                         //creates a new snake
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {x: x + 2, y});                         //third block  : x = 10 + 2, y = 10
        body.push_back(Block {x: x + 1, y});                         //second block : x = 10 + 1, y = 10
        body.push_back(Block {x, y});                                //first block  : x = 10,     y = 10
        //These values provide co-ordinates for the creation of a new snake equal to 3 blocks.

        Snake {
            direction: Direction::Right,                            // this is for the newly spawned snake.
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g); // this is not the creation of the block but the drawing and rendering of them. The co-ordinates and reference was given in funtion new(). However this function draws and reders the snake.
        }
    }

    pub fn head_position(&self) -> (i32, i32) {                // The head position of the snake.
        let head_block = self.body.front().unwrap();           // the front element of the linked list or None if now value.
        (head_block.x, head_block.y)
    }
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,             // sets the direction of the snake to d
            None => (),
        }
    
        let (last_x, last_y): (i32, i32) = self.head_position();
    
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,                            // so technically direction Up or Down doesnt change the X values but only moves on the y-axis. (for 2d games)
                y: last_y - 1,                        // in 2d games the origin is top left of screen. Therefore, positive y axis is downwards. Hence, to move Up you have to move across negative Y axis.
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,                       
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            }
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => (),
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
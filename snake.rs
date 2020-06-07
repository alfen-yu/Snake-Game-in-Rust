#![allow(dead_code)]

use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// use draw::draw_block;
 
const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.0];                   // snake color

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

struct Block {
    x: i32,
    y: i32,
}

struct Snake {                                                        //structure of a snake
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
    }

    Snake {
        direction: Direction::Right,
        body,
        tail: None,
    }
}
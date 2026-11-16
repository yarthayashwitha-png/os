//! In-Kernel Retro Arcade Snake Game (Bare-Metal VGA Mode)

use crate::vga::{WRITER, Color, BUFFER_WIDTH, BUFFER_HEIGHT};
use crate::keyboard::pop_key;
use crate::pit::sleep_ticks;

const MAX_SNAKE_LEN: usize = 200;
const GAME_WIDTH: usize = 50;
const GAME_HEIGHT: usize = 20;
const OFFSET_X: usize = 15;
const OFFSET_Y: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    x: usize,
    y: usize,
}

pub fn play() {
    let mut writer = WRITER.lock();
    writer.clear_screen();

    // Draw borders
    for x in 0..GAME_WIDTH {
        writer.write_char_at(OFFSET_X + x, OFFSET_Y, '#', Color::Yellow, Color::Black);
        writer.write_char_at(OFFSET_X + x, OFFSET_Y + GAME_HEIGHT - 1, '#', Color::Yellow, Color::Black);
    }
    for y in 0..GAME_HEIGHT {
        writer.write_char_at(OFFSET_X, OFFSET_Y + y, '#', Color::Yellow, Color::Black);
        writer.write_char_at(OFFSET_X + GAME_WIDTH - 1, OFFSET_Y + y, '#', Color::Yellow, Color::Black);
    }

    // Title & instructions
    let title = "=== AETHER-OS BARE-METAL ARCADE SNAKE ===";
    for (i, c) in title.chars().enumerate() {
        writer.write_char_at(18 + i, 0, c, Color::LightCyan, Color::Black);
    }
    let instructions = "Controls: [W/A/S/D] Move | [Q] Quit back to Shell";
    for (i, c) in instructions.chars().enumerate() {
        writer.write_char_at(16 + i, OFFSET_Y + GAME_HEIGHT + 1, c, Color::White, Color::Black);
    }

    drop(writer);

    let mut snake: [Point; MAX_SNAKE_LEN] = [
        Point { x: 10, y: 10 },
        Point { x: 9, y: 10 },
        Point { x: 8, y: 10 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
        Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 },
    ];
    let mut snake_len = 3;
    let mut dir = Direction::Right;
    let mut food = Point { x: 25, y: 10 };
    let mut score = 0;

    loop {
        // Handle input
        while let Some(ch) = pop_key() {
            match ch {
                'w' | 'W' if dir != Direction::Down => dir = Direction::Up,
                's' | 'S' if dir != Direction::Up => dir = Direction::Down,
                'a' | 'A' if dir != Direction::Right => dir = Direction::Left,
                'd' | 'D' if dir != Direction::Left => dir = Direction::Right,
                'q' | 'Q' => {
                    let mut w = WRITER.lock();
                    w.clear_screen();
                    return;
                }
                _ => {}
            }
        }

        // Calculate next head position
        let head = &snake[0];
        let next_head = match dir {
            Direction::Up => Point { x: head.x, y: if head.y > 1 { head.y - 1 } else { GAME_HEIGHT - 2 } },
            Direction::Down => Point { x: head.x, y: if head.y < GAME_HEIGHT - 2 { head.y + 1 } else { 1 } },
            Direction::Left => Point { x: if head.x > 1 { head.x - 1 } else { GAME_WIDTH - 2 }, y: head.y },
            Direction::Right => Point { x: if head.x < GAME_WIDTH - 2 { head.x + 1 } else { 1 }, y: head.y },
        };

        // Self-collision test
        let mut crash = false;
        for i in 0..snake_len {
            if snake[i].x == next_head.x && snake[i].y == next_head.y {
                crash = true;
                break;
            }
        }

        if crash {
            let mut w = WRITER.lock();
            let msg = ">>> GAME OVER! Press [Q] to return to shell <<<";
            for (i, c) in msg.chars().enumerate() {
                w.write_char_at(16 + i, OFFSET_Y + 10, c, Color::LightRed, Color::Black);
            }
            drop(w);
            loop {
                if let Some('q') | Some('Q') = pop_key() {
                    let mut w = WRITER.lock();
                    w.clear_screen();
                    return;
                }
                sleep_ticks(5);
            }
        }

        // Clear tail before move
        let tail = Point { x: snake[snake_len - 1].x, y: snake[snake_len - 1].y };
        {
            let mut w = WRITER.lock();
            w.write_char_at(OFFSET_X + tail.x, OFFSET_Y + tail.y, ' ', Color::Black, Color::Black);
        }

        // Move body
        for i in (1..snake_len).rev() {
            snake[i].x = snake[i - 1].x;
            snake[i].y = snake[i - 1].y;
        }
        snake[0] = next_head;

        // Check food collision
        if snake[0].x == food.x && snake[0].y == food.y {
            score += 10;
            if snake_len < MAX_SNAKE_LEN - 1 {
                snake_len += 1;
            }
            // Simple new food position
            food.x = 2 + ((food.x * 7 + 13) % (GAME_WIDTH - 4));
            food.y = 2 + ((food.y * 5 + 7) % (GAME_HEIGHT - 4));
        }

        // Render snake and food
        {
            let mut w = WRITER.lock();
            // Draw Food
            w.write_char_at(OFFSET_X + food.x, OFFSET_Y + food.y, '*', Color::LightRed, Color::Black);
            // Draw Snake Head
            w.write_char_at(OFFSET_X + snake[0].x, OFFSET_Y + snake[0].y, '@', Color::LightGreen, Color::Black);
            // Draw Snake Body
            for i in 1..snake_len {
                w.write_char_at(OFFSET_X + snake[i].x, OFFSET_Y + snake[i].y, 'o', Color::Green, Color::Black);
            }

            // Score display
            w.write_char_at(OFFSET_X + 2, OFFSET_Y + GAME_HEIGHT, 'S', Color::Yellow, Color::Black);
            w.write_char_at(OFFSET_X + 3, OFFSET_Y + GAME_HEIGHT, 'c', Color::Yellow, Color::Black);
            w.write_char_at(OFFSET_X + 4, OFFSET_Y + GAME_HEIGHT, 'o', Color::Yellow, Color::Black);
            w.write_char_at(OFFSET_X + 5, OFFSET_Y + GAME_HEIGHT, 'r', Color::Yellow, Color::Black);
            w.write_char_at(OFFSET_X + 6, OFFSET_Y + GAME_HEIGHT, 'e', Color::Yellow, Color::Black);
            w.write_char_at(OFFSET_X + 7, OFFSET_Y + GAME_HEIGHT, ':', Color::Yellow, Color::Black);
            w.write_char_at(OFFSET_X + 8, OFFSET_Y + GAME_HEIGHT, ' ', Color::Yellow, Color::Black);
            let s1 = ((score / 100) % 10) as u8 + b'0';
            let s2 = ((score / 10) % 10) as u8 + b'0';
            let s3 = (score % 10) as u8 + b'0';
            w.write_char_at(OFFSET_X + 9, OFFSET_Y + GAME_HEIGHT, s1 as char, Color::White, Color::Black);
            w.write_char_at(OFFSET_X + 10, OFFSET_Y + GAME_HEIGHT, s2 as char, Color::White, Color::Black);
            w.write_char_at(OFFSET_X + 11, OFFSET_Y + GAME_HEIGHT, s3 as char, Color::White, Color::Black);
        }

        // Delay ~100 ms (10 PIT ticks at 100Hz)
        sleep_ticks(10);
    }
}

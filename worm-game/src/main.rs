use macroquad::prelude::*;
use std::collections::VecDeque;

const CELL_SIZE: f32 = 50.0;
const BOARD_SIZE: i32 = 10;

const MOVE_INTERVAL: f32 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_title: "Worms".to_string(),
        window_height: 600,
        window_width: 600,
        window_resizable: false,
        ..Default::default()
    }
}

struct Board {
    w: f32,
    h: f32,
}

impl Board {
    fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }

    fn draw(&self) {
        draw_rectangle_lines(CELL_SIZE, CELL_SIZE, self.w, self.h, 3.0, GREEN);

        for row in 1..=10 {
            for col in 1..=10 {
                draw_rectangle_lines(
                    CELL_SIZE * col as f32,
                    CELL_SIZE * row as f32,
                    CELL_SIZE,
                    CELL_SIZE,
                    1.0,
                    DARKGRAY,
                );
            }
        }
    }
}

struct Cell {
    x: i32,
    y: i32,
    is_head: bool,
}

impl Cell {
    fn new(x: i32, y: i32, is_head: bool) -> Self {
        Self { x, y, is_head }
    }

    fn draw(&self) {
        draw_rectangle(
            self.x as f32 * CELL_SIZE,
            self.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            if self.is_head { RED } else { GREEN },
        );
    }
}

struct Worm {
    d: i32, // 0->up, 1->right, 2->down, 3->left
    cells: VecDeque<Cell>,
}

impl Worm {
    fn new() -> Self {
        Self {
            d: 2,
            cells: VecDeque::from([Cell::new(1, 1, true)]),
        }
    }

    fn step(&mut self) -> Cell {
        let mut prev_x = 0;
        let mut prev_y = 0;
        for (i, cell) in self.cells.iter_mut().enumerate() {
            // head의 경우
            if i == 0 {
                prev_x = cell.x;
                prev_y = cell.y;

                let (nx, ny) = match self.d {
                    0 => (0, -1),
                    1 => (1, 0),
                    2 => (0, 1),
                    3 => (-1, 0),
                    _ => unreachable!(),
                };

                cell.x += nx;
                cell.y += ny;
            }
            // body의 경우
            else {
                let now_x = cell.x;
                let now_y = cell.y;
                cell.x = prev_x;
                cell.y = prev_y;
                prev_x = now_x;
                prev_y = now_y;
            }
        }

        Cell::new(prev_x, prev_y, false)
    }

    fn can_move(&self) -> bool {
        // Board 밖
        let head = self.cells.front().unwrap();

        if head.x <= 0 || head.y <= 0 || 10 < head.x || 10 < head.y {
            return false;
        }

        if self
            .cells
            .iter()
            .skip(1)
            .any(|c| c.x == head.x && c.y == head.y)
        {
            return false;
        }
        true
    }

    fn draw(&self) {
        for cell in self.cells.iter() {
            cell.draw();
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn draw(&self) {
        draw_circle(
            self.x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            self.y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            10.0,
            YELLOW,
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let board = Board::new(CELL_SIZE * BOARD_SIZE as f32, CELL_SIZE * BOARD_SIZE as f32);
    let mut player = Worm::new();
    let mut point = Point::new(rand::gen_range(2, 10), rand::gen_range(2, 10));

    let mut step_time: f32 = 0.0;
    let mut next_d = 2;

    loop {
        // update
        // 이동
        step_time += get_frame_time();

        let interval = if is_key_down(KeyCode::Space) {
            MOVE_INTERVAL / 3.0
        } else {
            MOVE_INTERVAL
        };

        if step_time >= interval {
            step_time -= interval;

            player.d = next_d;

            let old_tail = player.step();

            let head = player.cells.front().unwrap();
            let player_x = head.x;
            let player_y = head.y;

            if head.x == point.x && head.y == point.y {
                player.cells.push_back(old_tail);

                let mut new_point_x = player_x;
                let mut new_point_y = player_y;

                while player
                    .cells
                    .iter()
                    .any(|c| c.x == new_point_x && c.y == new_point_y)
                {
                    new_point_x = rand::gen_range(1, 10);
                    new_point_y = rand::gen_range(1, 10);
                }

                point = Point::new(new_point_x, new_point_y);
            }
        }
        // 충돌
        if !player.can_move() {
            player = Worm::new();
        }

        // 방향 전환
        if is_key_pressed(KeyCode::Up) && !(player.d == 0 || player.d == 2) {
            next_d = 0;
        }

        if is_key_pressed(KeyCode::Right) && !(player.d == 1 || player.d == 3) {
            next_d = 1;
        }

        if is_key_pressed(KeyCode::Down) && !(player.d == 0 || player.d == 2) {
            next_d = 2;
        }

        if is_key_pressed(KeyCode::Left) && !(player.d == 1 || player.d == 3) {
            next_d = 3;
        }

        // draw
        clear_background(BLACK);
        board.draw();
        player.draw();
        point.draw();
        next_frame().await;
    }
}

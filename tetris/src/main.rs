use macroquad::{prelude::*, rand::gen_range};

const BOARD_ROWS: i32 = 20;
const BOARD_COLS: i32 = 10;
const BOARD_X: f32 = 50.0;
const BOARD_Y: f32 = 60.0;
const BLOCK_SIZE: f32 = 20.0;

const FALL_INTERVAL: f32 = 0.5;
const SOFT_DROP_INTERVAL: f32 = 0.15;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_string(),
        window_width: 50 * 2 + BLOCK_SIZE as i32 * BOARD_COLS,
        window_height: 60 * 2 + BLOCK_SIZE as i32 * BOARD_ROWS,
        window_resizable: false,
        ..Default::default()
    }
}

// =========== Shape Type =========
pub enum ShapeType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl ShapeType {
    pub fn color(&self) -> Color {
        match self {
            ShapeType::I => SKYBLUE,
            ShapeType::J => BLUE,
            ShapeType::L => ORANGE,
            ShapeType::O => YELLOW,
            ShapeType::S => GREEN,
            ShapeType::T => PURPLE,
            ShapeType::Z => RED,
        }
    }

    pub fn offset(&self) -> [(i32, i32); 4] {
        match self {
            ShapeType::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            ShapeType::J => [(-1, 0), (0, 0), (1, 0), (1, -1)],
            ShapeType::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
            ShapeType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            ShapeType::S => [(0, -1), (0, 0), (-1, 0), (-1, 1)],
            ShapeType::T => [(0, -1), (0, 0), (0, 1), (-1, 0)],
            ShapeType::Z => [(0, -1), (0, 0), (1, 0), (1, 1)],
        }
    }

    pub fn random() -> Self {
        match gen_range(1, 8) {
            1 => ShapeType::I,
            2 => ShapeType::J,
            3 => ShapeType::L,
            4 => ShapeType::O,
            5 => ShapeType::S,
            6 => ShapeType::T,
            7 => ShapeType::Z,
            _ => unreachable!(),
        }
    }
}

// ========= Shape =========
pub struct Shape {
    x: i32,
    y: i32,
    shape_type: ShapeType,
    rotation: usize,
}

impl Shape {
    pub fn new(x: i32, y: i32, shape_type: ShapeType) -> Self {
        Self {
            x,
            y,
            shape_type,
            rotation: 0,
        }
    }

    pub fn get_blocks(&self) -> [Block; 4] {
        let color = self.shape_type.color();
        let raw_offset = self.shape_type.offset();

        let mut blocks = [Block::new(0, 0, color); 4];

        for (i, &(ox, oy)) in raw_offset.iter().enumerate() {
            let (rx, ry) = match self.rotation % 4 {
                0 => (ox, oy),
                1 => (-oy, ox),
                2 => (-ox, -oy),
                3 => (oy, -ox),
                _ => unreachable!(),
            };

            blocks[i] = Block::new(self.x + rx, self.y + ry, color);
        }

        blocks
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn rotate(&mut self) {
        if !matches!(self.shape_type, ShapeType::O) {
            self.rotation = { self.rotation + 1 } % 4;
        }
    }

    pub fn can_move(&self, board: &Board, dx: i32, dy: i32) -> bool {
        self.get_blocks()
            .iter()
            .all(|b| !board.is_blocked(b.x + dx, b.y + dy))
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32, block_size: f32) {
        for block in self.get_blocks() {
            block.draw(offset_x, offset_y, block_size);
        }
    }
}

// =========== BLOCK ===========
#[derive(Copy, Clone, Debug)]
pub struct Block {
    x: i32,
    y: i32,
    color: Color,
}

impl Block {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32, block_size: f32) {
        let draw_x = offset_x + self.x as f32 * block_size;
        let draw_y = offset_y + self.y as f32 * block_size;

        draw_rectangle(
            draw_x,
            draw_y,
            block_size - 1.0,
            block_size - 1.0,
            self.color,
        );
    }
}

// ========= BOARD ==========
pub struct Board {
    w: f32,
    h: f32,
    cells: Vec<Vec<Option<Color>>>, // cells[y][x]
}

impl Board {
    fn new(w: f32, h: f32) -> Self {
        Self {
            w,
            h,
            cells: vec![vec![None; BOARD_COLS as usize]; BOARD_ROWS as usize],
        }
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= BOARD_COLS || y >= BOARD_ROWS {
            return true;
        }
        if y < 0 {
            return false;
        }
        self.cells[y as usize][x as usize].is_some()
    }

    fn lock(&mut self, shape: &Shape) {
        for b in shape.get_blocks() {
            if b.y >= 0 {
                self.cells[b.y as usize][b.x as usize] = Some(b.color);
            }
        }
    }

    fn clear(&mut self) {
        for row in self.cells.iter_mut() {
            row.fill(None);
        }
    }

    fn complete_check(&self) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for (y, row) in self.cells.iter().enumerate() {
            if row.iter().all(|b| b.is_some()) {
                result.push(y as i32);
            }
        }

        result
    }

    fn complete_by_line(&mut self, completed: Vec<i32>) {
        // 먼저 지우기
        for &row in completed.iter() {
            self.cells[row as usize].fill(None);
        }

        // 그 다음 내리기
        let mut write_y = BOARD_ROWS as usize;
        for read_y in (0..BOARD_ROWS as usize).rev() {
            if self.cells[read_y].iter().any(|c| c.is_some()) {
                write_y -= 1;
                if write_y != read_y {
                    self.cells.swap(write_y, read_y);
                }
            }
        }
    }

    fn draw(&self) {
        draw_rectangle_lines(BOARD_X, BOARD_Y, self.w, self.h, 2.0, WHITE);

        // 쌓인 블록 그리기
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(color) = cell {
                    Block::new(x as i32, y as i32, *color).draw(BOARD_X, BOARD_Y, BLOCK_SIZE);
                }
            }
        }
    }
}

// ======== Collision ========
// pub fn collision(shape: &Shape, board: &Board) -> bool

pub fn spawn_shape() -> Shape {
    Shape::new(5, 1, ShapeType::random())
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut score = 0;

    let mut board: Board = Board::new(
        BOARD_COLS as f32 * BLOCK_SIZE,
        BOARD_ROWS as f32 * BLOCK_SIZE,
    );
    let mut shape = spawn_shape();
    let mut fall_time = 0.0;

    let mut start = false;

    loop {
        if !start {
            clear_background(BLACK);
            draw_text(
                "TETRIS",
                (50.0 * 2.0 + BLOCK_SIZE * BOARD_COLS as f32) / 3.0,
                (60.0 * 2.0 + BLOCK_SIZE * BOARD_ROWS as f32) / 3.0,
                50.0,
                BLUE,
            );

            if is_key_pressed(KeyCode::Space) {
                start = true;
            }
            next_frame().await;
            continue;
        }

        // update
        let interval = if is_key_down(KeyCode::Down) {
            SOFT_DROP_INTERVAL
        } else {
            FALL_INTERVAL
        };

        // 하드 드롭
        if is_key_pressed(KeyCode::Space) {
            while shape.can_move(&board, 0, 1) {
                shape.move_by(0, 1);
            }
            fall_time = interval;
        }

        // 기본 낙하
        fall_time += get_frame_time();

        if fall_time >= interval {
            fall_time -= interval;

            if shape.can_move(&board, 0, 1) {
                shape.move_by(0, 1);
            } else {
                board.lock(&shape); // 바닥이나 블록 위에 닿으면 고정
                shape = spawn_shape(); // 새 블록 생성

                let completed: Vec<i32> = board.complete_check();
                score += (1000 * completed.len()) * completed.len();

                if !completed.is_empty() {
                    board.complete_by_line(completed);
                }

                // 새 블록이 스폰 위치부터 겹치면 보드 초기화(게임 오버)
                if !shape.can_move(&board, 0, 0) {
                    score = 0;
                    board.clear()
                }
            }
        }

        if is_key_pressed(KeyCode::Right) && shape.can_move(&board, 1, 0) {
            shape.move_by(1, 0);
        }

        if is_key_pressed(KeyCode::Left) && shape.can_move(&board, -1, 0) {
            shape.move_by(-1, 0);
        }

        if is_key_pressed(KeyCode::Up) {
            shape.rotate();
        }

        // draw
        clear_background(BLACK);

        draw_text(&format!("Score = {}", score), 20.0, 20.0, 20.0, WHITE);

        board.draw();
        shape.draw(BOARD_X, BOARD_Y, BLOCK_SIZE);

        next_frame().await;
    }
}

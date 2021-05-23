mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        //用于将Rust panic错误输出到JS console中的工具
        utils::set_panic_hook();
        let (width, height) = (64, 64);
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn is_cell_alive(&self, row: u32, column: u32) -> bool {
        let idx = self.get_index(row, column);
        self.cells[idx] == Cell::Alive
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn tick(&mut self) {
        // 复制一份细胞分布图用于下个迭代演化
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                // 存活状况统计
                let is_cell_alive = self.is_cell_alive(row, col);
                // 相邻存活数统计
                let live_neighbors = self.live_neighbor_count(row, col);
                // 状态机
                let next_cell = match (is_cell_alive, live_neighbors) {
                    // 1. 任何一个活细胞相邻活细胞数量少于2个将在下一个周期死亡
                    (true, x) if x < 2 => Cell::Dead,
                    // 2. 任何一个活细胞相邻活细胞数量有2到3个的在下一个周期存活
                    (true, 2) | (true, 3) => Cell::Alive,
                    // 3. 任何一个活细胞相邻活细胞数量多于3个将在下一个周期死亡
                    (true, x) if x > 3 => Cell::Dead,
                    // 4. 任何一个死细胞相邻活细胞数量等于3个将在下一个周期繁殖新生
                    (false, 3) => Cell::Alive,
                    // 其余死细胞继续保持死亡
                    _ => Cell::Dead,
                };
                let idx = self.get_index(row, col);
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn live_neighbor_count(&self, row: u32, column: u32) -> usize {
        // 遇到边界时应该溢出回到起点
        let (north, south, west, east) = (
            if row == 0 { self.height - 1 } else { row - 1 },
            if row == self.height - 1 { 0 } else { row + 1 },
            if column == 0 {
                self.width - 1
            } else {
                column - 1
            },
            if column == self.width - 1 {
                0
            } else {
                column + 1
            },
        );
        // 获取八个方向的坐标
        let position = [
            (north, column), // 北
            (south, column), // 南
            (row, west),     // 西
            (row, east),     // 东
            (north, west),   // 西北
            (north, east),   // 东北
            (south, west),   // 西南
            (south, east),   // 东南
        ];
        position
            .iter()
            .filter(|(x, y)| {
                let idx = self.get_index(*x, *y);
                self.cells[idx] == Cell::Alive
            })
            .count()
    }
}

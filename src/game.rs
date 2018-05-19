use cell::Cell::{self, *};

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
  pub width:  usize,
  pub height: usize,
  pub cells:  Vec<Cell>
}

impl Game {
  pub fn cell(&self, x: usize, y: usize) -> Cell {
    self.cells[self.index(x, y)]
  }

  fn index(&self, x: usize, y: usize) -> usize {
    x + y * self.width
  }

  fn neighbors(&self, i: usize) -> u8 {
    let mut neighbors = 0;

    let x = i % self.width;
    let y = (i - x) / self.height;

    let n = (y + self.height - 1) % self.height;
    let e = (x               + 1) % self.width;
    let s = (y               + 1) % self.height;
    let w = (x + self.width  - 1) % self.width;

    for y in &[n, y, s] {
      for x in &[w, x, e] {
        let ni = self.index(*x, *y);
      
        if ni == i {
          continue;
        }

        if self.cells[ni] == Alive {
          neighbors += 1;
        }
      }
    }

    neighbors
  }

  pub fn tick(&self) -> Game {
    let cells = self.cells.iter().enumerate().map(|(i, cell)| {
      cell.tick(self.neighbors(i))
    }).collect();

    Game {
      cells,
      ..*self
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! cell {
    (1) => {Alive};
    (0) => {Dead };
  }

  macro_rules! game {
    ($([$($cell:tt)*])*) => {
      {
        let mut rows: Vec<Vec<Cell>> = Vec::new();
        $(
        {
          let mut row: Vec<Cell> = Vec::new();
          $(
          {
            row.push(cell!($cell));
          }
          )*
          rows.push(row);
        }
        )*

        let len = rows.get(0)
          .map(|row| row.len())
          .unwrap_or(0);

        rows.iter().for_each(|row| if row.len() != len {
          panic!("row length mismatch");
        });

        let height = rows.len();
        let width  = rows.get(0).map(|row| row.len()).unwrap_or(0);
        let cells  = rows.into_iter()
          .flat_map(|row| row.into_iter())
          .collect();

        Game {
          width,
          height,
          cells,
        }
      }
    };
  }

  #[test]
  fn small() {
    let mut game = game! {
      [0]
    };

    for _ in 0..10 {
      game = game.tick();
    }
  }

  #[test]
  fn medium() {
    let mut game = game! {
      [0 0]
    };

    for _ in 0..10 {
      game = game.tick();
    }
  }

  #[test]
  fn blink() {
    let vertical = game! {
      [0 0 0 0 0]
      [0 0 1 0 0]
      [0 0 1 0 0]
      [0 0 1 0 0]
      [0 0 0 0 0]
    };

    let horizontal = game! {
      [0 0 0 0 0]
      [0 0 0 0 0]
      [0 1 1 1 0]
      [0 0 0 0 0]
      [0 0 0 0 0]
    };

    let mut game = vertical.clone();

    for i in 0..10 {
      if i % 2 == 0 {
        assert_eq!(game, vertical);
      } else {
        assert_eq!(game, horizontal);
      }
      game = game.tick();
    }
  }

  #[test]
  fn block() {
    let block = game! {
      [0 0 0 0]
      [0 1 1 0]
      [0 1 1 0]
      [0 0 0 0]
    };

    let mut game = block.clone();
    for _ in 0..10 {
      game = game.tick();
      assert_eq!(game, block);
    }
  }

  #[test]
  fn wrap() {
    let block = game! {
      [1 0 0 1]
      [0 0 0 0]
      [0 0 0 0]
      [1 0 0 1]
    };

    let mut game = block.clone();
    for _ in 0..10 {
      game = game.tick();
      assert_eq!(game, block);
    }
  }

  #[test]
  fn blinkers() {
    let horizontal = game! {
      [0 0 0 0 0 0 0 0 0 0]
      [0 1 0 0 0 0 0 0 1 0]
      [0 1 0 0 0 0 0 0 1 0]
      [0 1 0 0 0 0 0 0 1 0]
      [0 0 0 0 0 0 0 0 0 0]
      [0 0 0 0 0 0 0 0 0 0]
      [0 1 0 0 0 0 0 0 1 0]
      [0 1 0 0 0 0 0 0 1 0]
      [0 1 0 0 0 0 0 0 1 0]
      [0 0 0 0 0 0 0 0 0 0]
    };

    let vertical = game! {
      [0 0 0 0 0 0 0 0 0 0]
      [0 0 0 0 0 0 0 0 0 0]
      [1 1 1 0 0 0 0 1 1 1]
      [0 0 0 0 0 0 0 0 0 0]
      [0 0 0 0 0 0 0 0 0 0]
      [0 0 0 0 0 0 0 0 0 0]
      [0 0 0 0 0 0 0 0 0 0]
      [1 1 1 0 0 0 0 1 1 1]
      [0 0 0 0 0 0 0 0 0 0]
      [0 0 0 0 0 0 0 0 0 0]
    };

    assert_eq!(horizontal.tick(), vertical);
  }
}

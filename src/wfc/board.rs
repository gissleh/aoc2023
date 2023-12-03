pub trait Board: Sized {
    fn board_size(&self) -> (usize, usize);
    fn board_cell(&self, x: usize, y: usize) -> Option<&u64>;
    fn board_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut u64>;

    #[inline]
    fn board_has_contradictions(&self) -> bool {
        let (w, h) = self.board_size();
        for y in 0..h {
            for x in 0..w {
                if let Some(cell) = self.board_cell(x, y) {
                    if *cell == 0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    #[inline]
    fn full_propagate<F: Fn(u64, usize, usize) -> u64>(
        &mut self,
        except_x: usize,
        except_y: usize,
        f: F,
    ) {
        let (w, h) = self.board_size();
        for y in 0..h {
            let except = y == except_y;
            for x in 0..w {
                if except && x == except_x {
                    continue;
                }
                if let Some(cell) = self.board_cell_mut(x, y) {
                    *cell = f(*cell, x, y)
                }
            }
        }
    }
}

pub struct UnFlattenedBoard<B: Board>(pub B, pub usize);

impl<B: Board> Board for UnFlattenedBoard<B> {
    #[inline]
    fn board_size(&self) -> (usize, usize) {
        let (w, _) = self.0.board_size();
        (w % self.1, w / self.1)
    }

    #[inline]
    fn board_cell(&self, x: usize, y: usize) -> Option<&u64> {
        self.0.board_cell(x + (y * self.1), 1)
    }

    #[inline]
    fn board_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut u64> {
        self.0.board_cell_mut(x + (y * self.1), 1)
    }
}

impl<const S: usize> Board for [u64; S] {
    #[inline]
    fn board_size(&self) -> (usize, usize) {
        (self.len(), 1)
    }

    #[inline]
    fn board_cell(&self, x: usize, y: usize) -> Option<&u64> {
        if y == 0 {
            self.get(x)
        } else {
            None
        }
    }

    #[inline]
    fn board_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut u64> {
        if y == 0 {
            self.get_mut(x)
        } else {
            None
        }
    }

    fn full_propagate<F: Fn(u64, usize, usize) -> u64>(
        &mut self,
        except_x: usize,
        except_y: usize,
        f: F,
    ) {
        let except = except_y == 0;
        for (i, v) in self.iter_mut().enumerate() {
            if except && i == except_x {
                continue;
            }

            *v = f(*v, i, 0);
        }
    }
}

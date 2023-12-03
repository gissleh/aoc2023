use std::marker::PhantomData;

mod board;
mod util;

pub use board::*;
pub use util::*;

#[derive(Eq, PartialEq, Debug)]
pub enum RunResult {
    Continue,
    Done,
    Contradicted,
    UnCollapsible,
}

pub trait Rule<B: Board>: Sized {
    fn select(&self, board: &B) -> Option<(usize, usize)>;
    fn collapse(&self, board: &mut B, x: usize, y: usize) -> bool;
    fn propagate(&self, board: &mut B, x: usize, y: usize, c: u64) -> bool;

    #[inline]
    fn run(&self, board: &mut B) -> RunResult {
        if let Some((x, y)) = self.select(board) {
            if !self.collapse(board, x, y) {
                return RunResult::UnCollapsible;
            }

            let c = *board.board_cell(x, y).unwrap();
            self.propagate(board, x, y, c);

            if board.board_has_contradictions() {
                RunResult::Contradicted
            } else {
                RunResult::Continue
            }
        } else {
            RunResult::Done
        }
    }

    #[inline]
    fn and<R2: Rule<B>>(self, r2: R2) -> ConjunctionRule<B, Self, R2> {
        ConjunctionRule {
            r1: self,
            r2,
            spooky: PhantomData::default(),
        }
    }
}

pub struct ConjunctionRule<B: Board, R1: Rule<B>, R2: Rule<B>> {
    r1: R1,
    r2: R2,
    spooky: PhantomData<B>,
}

impl<B: Board, R1: Rule<B>, R2: Rule<B>> Rule<B> for ConjunctionRule<B, R1, R2> {
    fn select(&self, board: &B) -> Option<(usize, usize)> {
        self.r1.select(board).or_else(|| self.r2.select(board))
    }

    fn collapse(&self, board: &mut B, x: usize, y: usize) -> bool {
        self.r1.collapse(board, x, y) || self.r2.collapse(board, x, y)
    }

    fn propagate(&self, board: &mut B, x: usize, y: usize, c: u64) -> bool {
        let p1 = self.r1.propagate(board, x, y, c);
        let p2 = self.r2.propagate(board, x, y, c);

        p1 || p2
    }
}

struct SelectorFN<B: Board, F: Fn(&B) -> Option<(usize, usize)>>(F, PhantomData<B>);

impl<B: Board, F: Fn(&B) -> Option<(usize, usize)>> Rule<B> for SelectorFN<B, F> {
    #[inline]
    fn select(&self, board: &B) -> Option<(usize, usize)> {
        self.0(board)
    }
    #[inline]
    fn collapse(&self, _board: &mut B, _x: usize, _y: usize) -> bool {
        false
    }
    #[inline]
    fn propagate(&self, _board: &mut B, _x: usize, _y: usize, _c: u64) -> bool {
        false
    }
}

pub fn selector<B: Board, F: Fn(&B) -> Option<(usize, usize)>>(f: F) -> impl Rule<B> {
    SelectorFN(f, Default::default())
}

struct CollapserFN<B: Board, F: Fn(&mut B, usize, usize) -> bool>(F, PhantomData<B>);

impl<B: Board, F: Fn(&mut B, usize, usize) -> bool> Rule<B> for CollapserFN<B, F> {
    #[inline]
    fn select(&self, _board: &B) -> Option<(usize, usize)> {
        None
    }
    #[inline]
    fn collapse(&self, board: &mut B, x: usize, y: usize) -> bool {
        self.0(board, x, y)
    }
    #[inline]
    fn propagate(&self, _board: &mut B, _x: usize, _y: usize, _c: u64) -> bool {
        false
    }
}

pub fn collapser<B: Board, F: Fn(&mut B, usize, usize) -> bool>(f: F) -> impl Rule<B> {
    CollapserFN(f, Default::default())
}

struct PropagatorFN<B: Board, F: Fn(&mut B, usize, usize, u64) -> bool>(F, PhantomData<B>);

impl<B: Board, F: Fn(&mut B, usize, usize, u64) -> bool> Rule<B> for PropagatorFN<B, F> {
    #[inline]
    fn select(&self, _board: &B) -> Option<(usize, usize)> {
        None
    }
    #[inline]
    fn collapse(&self, _board: &mut B, _x: usize, _y: usize) -> bool {
        false
    }
    #[inline]
    fn propagate(&self, board: &mut B, x: usize, y: usize, c: u64) -> bool {
        self.0(board, x, y, c)
    }
}

pub fn propagator<B: Board, F: Fn(&mut B, usize, usize, u64) -> bool>(f: F) -> impl Rule<B> {
    PropagatorFN(f, Default::default())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn there_can_be_only_one() {
        let rule = select_first_lowest::<[u64; 4]>()
            .and(collapser(|b: &mut [u64; 4], x, y| {
                let c = b.board_cell_mut(x, y).unwrap();
                *c = 1 << c.trailing_zeros();
                true
            }))
            .and(propagator(|b: &mut [u64; 4], x, y, c| {
                b.full_propagate(x, y, |v, _, _| v & !c);
                true
            }));

        let mut board = [0b1110, 0b1110, 0b1110, 0b0001];

        assert_eq!(rule.select(&board), Some((0, 0)));
        assert_eq!(rule.run(&mut board), RunResult::Continue);
        assert_eq!(&board, &[0b0010, 0b1100, 0b1100, 0b0001]);
        assert_eq!(rule.run(&mut board), RunResult::Continue);
        assert_eq!(&board, &[0b0010, 0b0100, 0b1000, 0b0001]);
        assert_eq!(rule.run(&mut board), RunResult::Done);
    }
}

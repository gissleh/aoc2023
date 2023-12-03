use crate::wfc::{selector, Board, Rule};

pub fn select_first_lowest<B: Board>() -> impl Rule<B> {
    selector(|b: &B| {
        let (w, h) = b.board_size();
        let mut lowest_index = None::<(usize, usize)>;
        let mut lowest_entropy = 64;

        for y in 0..h {
            for x in 0..w {
                let entropy = b.board_cell(x, y).unwrap().count_ones();
                if entropy > 1 && entropy < lowest_entropy {
                    lowest_index = Some((x, y));
                    lowest_entropy = entropy;
                }
            }
        }

        lowest_index
    })
}

use common::aoc::Day;
use common::parse;
use common::parse::{Parser};

pub fn main(day: &mut Day, input: &[u8]) {
    let input: Vec<Hand> = day.prep("Parse", || Hand::parser().repeat().parse(input).unwrap());

    day.note("Amount of hands", input.len());

    day.part("Part 1", || p1(&input));
    let input: Vec<Hand> = day.prep("Convert J to 1", || input.iter().map(|h| h.convert()).collect());
    day.part("Part 2", || p2(&input));
}

fn p1(hands: &[Hand]) -> u32 {
    let mut levels: Vec<(u32, usize)> = hands.iter().enumerate().map(|(i, h)| (h.level(), i)).collect();

    levels.sort_by(|(al, ai), (bl, bi)| {
        al.cmp(bl).then_with(|| hands[*ai].cards.cmp(&hands[*bi].cards))
    });

    levels.iter()
        .enumerate()
        .map(|(i, (_, index))| hands[*index].bid * (i + 1) as u32)
        .sum()
}

fn p2(hands: &[Hand]) -> u32 {
    let mut levels: Vec<(u32, usize)> = hands.iter().enumerate().map(|(i, h)| (h.joker_level(), i)).collect();

    levels.sort_by(|(al, ai), (bl, bi)| {
        al.cmp(bl).then_with(|| hands[*ai].cards.cmp(&hands[*bi].cards))
    });

    levels.iter()
        .enumerate()
        .map(|(i, (_, index))| hands[*index].bid * (i + 1) as u32)
        .sum()
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    #[inline]
    fn level(&self) -> u32 {
        let mut counts = [0u8; 15];
        for card in self.cards.iter() {
            counts[*card as usize] += 1;
        }

        counts.sort_unstable();
        let highest = counts[14];
        let second = counts[13];

        match highest {
            5 => 7,
            4 => 6,
            3 if second == 2 => 5,
            3 => 4,
            2 if second == 2 => 3,
            2 => 2,
            _ => 1,
        }
    }

    fn joker_level(&self) -> u32 {
        let mut counts = [0u8; 15];
        let mut jokers = 0u8;
        for card in self.cards.iter() {
            if *card == 1 {
                jokers += 1;
            } else {
                counts[*card as usize] += 1;
            }
        }

        counts.sort_unstable();
        let highest = counts[14];
        let second = counts[13];

        match (highest, jokers) {
            (5, 0) => 7,
            (4, 1) => 7,
            (3, 2) => 7,
            (2, 3) => 7,
            (1, 4) => 7,
            (0, 5) => 7,
            (4, 0) => 6,
            (3, 1) => 6,
            (2, 2) => 6,
            (1, 3) => 6,
            (0, 4) => 6,
            (3, 0) if second == 2 => 5,
            (2, 1) if second == 2 => 5,
            (1, 2) if second == 2 => 5,
            (3, 0) => 4,
            (2, 1) => 4,
            (1, 2) => 4,
            (2, 0) if second == 2 => 3,
            (1, 1) if second == 2 => 3,
            (2, 0) => 2,
            (1, 1) => 2,
            (1, 0) => 1,
            _ => panic!("UNHANDLED {} {}", highest, jokers),
        }
    }

    fn convert(&self) -> Hand {
        let mut h = *self;
        for i in 0..5 {
            if h.cards[i] == 11 {
                h.cards[i] = 1;
            }
        }

        h
    }

    #[inline]
    fn new(cards: [u8; 5], bid: u32) -> Self {
        Self{cards, bid}
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::choice((
            parse::digit(),
            b'T'.map_to(10),
            b'J'.map_to(11),
            b'Q'.map_to(12),
            b'K'.map_to(13),
            b'A'.map_to(14),
        ))
            .repeat_n(5)
            .and_discard(b' ')
            .and(parse::unsigned_int())
            .then_skip(b'\n')
            .map(|(cards, bid)| Hand::new(cards, bid) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    fn h(s: &[u8]) -> Hand {
        Hand::parser().parse(s).unwrap()
    }

    #[test]
    fn card_levels_are_correct() {
        assert_eq!(h(b"AAAAA 0").level(), 7);
        assert_eq!(h(b"AAAA3 0").level(), 6);
        assert_eq!(h(b"AAA77 0").level(), 5);
        assert_eq!(h(b"66444 0").level(), 5);
        assert_eq!(h(b"22215 0").level(), 4);
        assert_eq!(h(b"11223 0").level(), 3);
        assert_eq!(h(b"549TT 0").level(), 2);
        assert_eq!(h(b"12345 0").level(), 1);
    }

    #[test]
    fn card_joker_levels_are_correct() {
        assert_eq!(h(b"32T3K 0").convert().joker_level(), 2);
        assert_eq!(h(b"T55J5 0").convert().joker_level(), 6);
        assert_eq!(h(b"KK677 0").convert().joker_level(), 3);
        assert_eq!(h(b"KTJJT 0").convert().joker_level(), 6);
        assert_eq!(h(b"QQQJA 0").convert().joker_level(), 6);

        assert_eq!(h(b"AAAA2 0").convert().joker_level(), 6);
        assert_eq!(h(b"JAAA2 0").convert().joker_level(), 6);
        assert_eq!(h(b"AJJA2 0").convert().joker_level(), 6);
        assert_eq!(h(b"JJAJ2 0").convert().joker_level(), 6);
        assert_eq!(h(b"JJJJ2 0").convert().joker_level(), 7);
        assert_eq!(h(b"JJ333 0").convert().joker_level(), 7);
        assert_eq!(h(b"AKKJA 0").convert().joker_level(), 5);
        assert_eq!(h(b"AKKJK 0").convert().joker_level(), 6);
        assert_eq!(h(b"K345J 0").convert().joker_level(), 2);
        assert_eq!(h(b"AA37J 0").convert().joker_level(), 4);
    }

    #[test]
    fn p1_works_on_example() {
        let hands: Vec<Hand> = Hand::parser().repeat().parse(EXAMPLE).unwrap();

        assert_eq!(p1(&hands), 6440);
    }

    #[test]
    fn p2_works_on_example() {
        let mut hands: Vec<Hand> = Hand::parser().repeat().parse(EXAMPLE).unwrap();
        for h in hands.iter_mut() {
            *h = h.convert();
        }

        assert_eq!(p2(&hands), 5905);
    }
}
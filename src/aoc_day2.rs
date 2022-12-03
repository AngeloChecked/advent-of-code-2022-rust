use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum RoundResult {
    Win,
    Lost,
    Tie,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Round {
    symbol: Hand,
    result: RoundResult,
}

impl Round {
    pub fn from_request((left, request_output): (Hand, RoundResult)) -> Round {
        match (left, &request_output) {
            (Hand::Rock, RoundResult::Win) => Round::new(Hand::Paper, request_output),
            (Hand::Rock, RoundResult::Lost) => Round::new(Hand::Scissor, request_output),
            (Hand::Rock, RoundResult::Tie) => Round::new(Hand::Rock, request_output),
            (Hand::Paper, RoundResult::Win) => Round::new(Hand::Scissor, request_output),
            (Hand::Paper, RoundResult::Lost) => Round::new(Hand::Rock, request_output),
            (Hand::Paper, RoundResult::Tie) => Round::new(Hand::Paper, request_output),
            (Hand::Scissor, RoundResult::Win) => Round::new(Hand::Rock, request_output),
            (Hand::Scissor, RoundResult::Lost) => Round::new(Hand::Paper, request_output),
            (Hand::Scissor, RoundResult::Tie) => Round::new(Hand::Scissor, request_output),
        }
    }
    pub fn from_left_hand((left, right): (Hand, Hand)) -> Round {
        match (left, right) {
            (Hand::Rock, Hand::Rock) => Round::new(Hand::Rock, RoundResult::Tie),
            (Hand::Rock, Hand::Scissor) => Round::new(Hand::Rock, RoundResult::Win),
            (Hand::Rock, Hand::Paper) => Round::new(Hand::Rock, RoundResult::Lost),
            (Hand::Scissor, Hand::Rock) => Round::new(Hand::Scissor, RoundResult::Lost),
            (Hand::Scissor, Hand::Scissor) => Round::new(Hand::Scissor, RoundResult::Tie),
            (Hand::Scissor, Hand::Paper) => Round::new(Hand::Scissor, RoundResult::Win),
            (Hand::Paper, Hand::Rock) => Round::new(Hand::Paper, RoundResult::Win),
            (Hand::Paper, Hand::Scissor) => Round::new(Hand::Paper, RoundResult::Lost),
            (Hand::Paper, Hand::Paper) => Round::new(Hand::Paper, RoundResult::Tie),
        }
    }
    pub fn from_right_hand((left, right): (Hand, Hand)) -> Round {
        Round::from_left_hand((right, left))
    }
    pub fn new(symbol: Hand, result: RoundResult) -> Round {
        Round { symbol, result }
    }
    pub fn points(self) -> usize {
        (find(&HAND_POINTS, self.symbol) + find(&ROUND_POINTS, self.result)) as usize
    }
}

pub const LEFT_SYMBOLS: [(&str, Hand); 3] =
    [("A", Hand::Rock), ("B", Hand::Paper), ("C", Hand::Scissor)];
pub const RIGHT_SYMBOLS: [(&str, Hand); 3] =
    [("X", Hand::Rock), ("Y", Hand::Paper), ("Z", Hand::Scissor)];

pub const ROUND_END_REQUEST_SYMBOLS: [(&str, RoundResult); 3] = [
    ("X", RoundResult::Lost),
    ("Y", RoundResult::Tie),
    ("Z", RoundResult::Win),
];

pub const HAND_POINTS: [(Hand, i32); 3] = [(Hand::Rock, 1), (Hand::Paper, 2), (Hand::Scissor, 3)];
pub const ROUND_POINTS: [(RoundResult, i32); 3] = [
    (RoundResult::Win, 6),
    (RoundResult::Lost, 0),
    (RoundResult::Tie, 3),
];

fn find<T, B>(list: &[(T, B)], key: T) -> B
where
    T: Debug,
    T: Eq,
    T: PartialEq,
    B: Debug,
    B: Clone,
{
    list.iter()
        .find(|(innkey, _)| *innkey == key)
        .unwrap()
        .1
        .clone()
}

pub fn parse_round_inputs(raw_rounds: Vec<String>) -> Vec<(Hand, Hand)> {
    let mut result = Vec::new();
    for raw_round in raw_rounds {
        let players_hands_raw: Vec<&str> = raw_round.split(" ").collect();
        let left_hand = find(&LEFT_SYMBOLS, players_hands_raw.get(0).unwrap());
        let right_hand = find(&RIGHT_SYMBOLS, players_hands_raw.get(1).unwrap());
        result.push((left_hand, right_hand));
    }
    result
}

pub fn solution(raw_rounds: Vec<String>) -> usize {
    let rounds_inputs = parse_round_inputs(raw_rounds);
    rounds_inputs
        .into_iter()
        .map(Round::from_right_hand)
        .map(Round::points)
        .sum()
}

pub fn parse_round_requests(raw_rounds: Vec<String>) -> Vec<(Hand, RoundResult)> {
    let mut result = Vec::new();
    for raw_round in raw_rounds {
        let players_hands_raw: Vec<&str> = raw_round.split(" ").collect();
        let left_hand = find(&LEFT_SYMBOLS, players_hands_raw.get(0).unwrap());
        let round_request = find(
            &ROUND_END_REQUEST_SYMBOLS,
            players_hands_raw.get(1).unwrap(),
        );
        result.push((left_hand, round_request));
    }
    result
}

pub fn solution_part2(raw_rounds: Vec<String>) -> usize {
    let rounds_inputs = parse_round_requests(raw_rounds);
    rounds_inputs
        .into_iter()
        .map(Round::from_request)
        .map(Round::points)
        .sum()
}

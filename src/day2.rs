// Rock Paper Scissors

use std::{borrow::Borrow, collections::HashMap, io::BufRead};

use crate::get_buffer;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum GameResult {
    Win,
    Draw,
    Lose,
}

struct GamePoints {
    p1: usize,
    p2: usize,
}

fn get_player_move_from_result(p1_move: &PlayerMove, game_res: &GameResult) -> PlayerMove {
    match game_res {
        GameResult::Draw => p1_move.clone(),
        GameResult::Win => match p1_move {
            PlayerMove::Rock => PlayerMove::Paper,
            PlayerMove::Paper => PlayerMove::Scissors,
            PlayerMove::Scissors => PlayerMove::Rock,
        },
        GameResult::Lose => match p1_move {
            PlayerMove::Paper => PlayerMove::Rock,
            PlayerMove::Scissors => PlayerMove::Paper,
            PlayerMove::Rock => PlayerMove::Scissors,
        },
    }
}

fn calculate_points(p1: PlayerMove, p2: PlayerMove) -> GamePoints {
    let shape_point = [PlayerMove::Rock, PlayerMove::Paper, PlayerMove::Scissors];
    let get_shape_point =
        |x: &PlayerMove| shape_point.iter().enumerate().find(|y| y.1 == x).unwrap().0 + 1;
    let mut gp = GamePoints {
        p1: get_shape_point(&p1),
        p2: get_shape_point(&p2),
    };
    let to_add = match (p1, p2) {
        (PlayerMove::Rock, PlayerMove::Scissors)
        | (PlayerMove::Scissors, PlayerMove::Paper)
        | (PlayerMove::Paper, PlayerMove::Rock) => (6, 0),
        (PlayerMove::Scissors, PlayerMove::Rock)
        | (PlayerMove::Paper, PlayerMove::Scissors)
        | (PlayerMove::Rock, PlayerMove::Paper) => (0, 6),
        _ => (3, 3),
    };
    gp.p1 += to_add.0;
    gp.p2 += to_add.1;
    gp
}

pub struct Day2 {}
impl Day2 {
    fn parse<T: Clone>(player_2_map: HashMap<&str, T>) -> Vec<(PlayerMove, T)> {
        let buf = get_buffer("input/day2.txt");
        let player_1_map = [
            ("A", PlayerMove::Rock),
            ("B", PlayerMove::Paper),
            ("C", PlayerMove::Scissors),
        ]
        .into_iter()
        .collect::<HashMap<&str, PlayerMove>>();

        buf.lines()
            .map(|x| x.unwrap())
            .map(|x| x.split(' ').map(|x| x.to_owned()).collect::<Vec<String>>())
            .map(|x| {
                (
                    player_1_map
                        .get(x.get(0).unwrap().as_str())
                        .unwrap()
                        .clone(),
                    player_2_map
                        .get(x.get(1).unwrap().as_str())
                        .unwrap()
                        .clone(),
                )
            })
            .collect::<Vec<(PlayerMove, T)>>()
    }

    pub fn part_1() -> usize {
        let player_2_map = [
            ("X", PlayerMove::Rock),
            ("Y", PlayerMove::Paper),
            ("Z", PlayerMove::Scissors),
        ]
        .into_iter()
        .collect::<HashMap<&str, PlayerMove>>();
        let parsed_inp = Day2::parse(player_2_map);
        let game_points = parsed_inp
            .iter()
            .map(|x| calculate_points(x.0.clone(), x.1.clone()))
            .collect::<Vec<GamePoints>>();
        game_points.iter().map(|x| x.p2).sum()
    }

    pub fn part_2() -> usize {
        let player_2_map = [
            ("X", GameResult::Lose),
            ("Y", GameResult::Draw),
            ("Z", GameResult::Win),
        ]
        .into_iter()
        .collect::<HashMap<&str, GameResult>>();
        let parsed_input = Day2::parse(player_2_map);
        parsed_input
            .iter()
            .map(|x| calculate_points(x.0.clone(), get_player_move_from_result(&x.0, &x.1)))
            .map(|x| x.p2)
            .sum()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day2;
        println!("{:?}", Day2::part_1());
        println!("{:?}", Day2::part_2());
    }
}

enum Balls {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Balls {
    fn from(ball: &str) -> Self {
        let ball = ball.trim();
        let (count, ball_color) = ball.split_once(' ').unwrap();
        let count = count.parse().unwrap();
        match ball_color {
            "red" => Balls::Red(count),
            "blue" => Balls::Blue(count),
            "green" => Balls::Green(count),
            _ => panic!("{}", ball_color),
        }
    }
}

#[derive(Default)]
struct Game {
    id: u32,
    pub red_balls: u32,
    pub green_balls: u32,
    pub blue_balls: u32,
}

impl Game {
    pub fn process_game(game: &str) -> Self {
        let mut g = 0;
        let mut r = 0;
        let mut b = 0;

        let (game_ident, rounds) = game.split_once(':').unwrap();
        let id = game_ident.split_once(' ').unwrap().1.parse().unwrap();

        for round in rounds.split(';') {
            for ball in round.split(',') {
                match Balls::from(ball) {
                    Balls::Blue(c) => b = b.max(c),
                    Balls::Green(c) => g = g.max(c),
                    Balls::Red(c) => r = r.max(c),
                }
            }
        }

        Self {
            id,
            red_balls: r,
            green_balls: g,
            blue_balls: b,
        }
    }

    fn game_pos(&self) -> u32 {
        if self.red_balls <= 12 && self.blue_balls <= 14 && self.green_balls <= 13 {
            return self.id;
        }
        0
    }
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for game in input.split('\n') {
        sum += Game::process_game(game).game_pos();
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for game in input.split('\n') {
        let t = Game::process_game(game);
        sum += t.red_balls * t.blue_balls * t.green_balls;
    }

    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let part = args.get(1);
    let part = match part {
        Some(d) => d,
        None => panic!("Specify the part you want to run {args:?}"),
    };

    let input = include_str!("../inputs/day2/input.txt");
    let before = std::time::Instant::now();
    let result = match part.as_str() {
        "part1" => part1(input),
        "part2" => part2(input),
        _ => panic!("Specify one of 2 parts {part}"),
    };

    println!("The result is {result} duration:{:?}", before.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        assert_eq!(part1(include_str!("../inputs/day2/part1_sample.txt")), 8)
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(include_str!("../inputs/day2/part1_sample.txt")), 2286)
    }
}

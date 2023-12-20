use crate::*;

// This is not working

impl_day!(DayTwo);

struct Tally {
    red: u32,
    green: u32,
    blue: u32,
}

impl Tally {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl DayTwo {
    fn solution(&self) -> u32 {
        let games: Vec<String> = self
            .file_contents
            .to_owned()
            .iter()
            .map(|line| line.split(": ").last().unwrap_or_default().to_string())
            .collect();

        let mut total: u32 = 0;
        for (i, game) in games.iter().enumerate() {
            let sets = game.split("; ");
            let mut tally = Tally::new();

            for set in sets {
                let cubes = set.split(", ");
                for cube in cubes {
                    let mut data = cube.split(' ');
                    let (amount, colour) = (
                        data.next().unwrap().parse::<u32>().unwrap_or_default(),
                        data.next().unwrap(),
                    );

                    match colour {
                        "red" => tally.red += amount,
                        "green" => tally.green += amount,
                        "blue" => tally.blue += amount,
                        _ => unreachable!("This should never happen!"),
                    }
                }
            }

            if tally.red <= 12 && tally.green <= 13 && tally.blue <= 14 {
                total += (i + 1) as u32;
            } 

            println!("{}", game);
        }

        total
    }
}

impl Solve for DayTwo {
    fn solve(&self) -> String {
        self.solution().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
       let day2 = day2::DayTwo {
           file_contents : vec![
               "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
               "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
               "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
               "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
               "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
           ]
       }; 

       assert_eq!(day2.solution(), 8);
    }
}

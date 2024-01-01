use std::fs;

fn part1() {
    let file = fs::read_to_string("./src/input.txt").expect("Unable to read from file");
    let lines = file.lines();

    let mut final_sum = 0;

    'outer: for line in lines {
        let game_split: Vec<&str> = line.split(": ").collect();
        let curr_game = game_split[0].replace("Game ", "");
        let game_str = game_split[1];
        let takes = game_str.split(";");

        for take in takes {
            let colors_split: Vec<&str> = take.split(",").collect();
            for color in colors_split {
                if color.contains("blue") {
                    let blue_split: Vec<&str> = color.split("blue").collect();
                    let blue_num = blue_split[0].trim().parse::<i8>().unwrap();
                    if blue_num > 14 {
                        continue 'outer;
                    }
                }
                else if color.contains("red") {
                    let red_split: Vec<&str> = color.split("red").collect();
                    let red_num = red_split[0].trim().parse::<i8>().unwrap();
                    if red_num > 12 {
                        continue 'outer;
                    }
                }
                else if color.contains("green") {
                    let green_split: Vec<&str> = color.split("green").collect();
                    let green_num = green_split[0].trim().parse::<i8>().unwrap();
                    if green_num > 13 {
                        continue 'outer;
                    }
                }
            }
        }
        final_sum += curr_game.parse::<i32>().unwrap();
    }
    println!("Final sum: {}", final_sum);
}

fn main() {
    part1();
}

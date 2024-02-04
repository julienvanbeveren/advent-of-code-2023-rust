use std::fs;

fn part1() {
    let file = fs::read_to_string("./src/input.txt").expect("Unable to read from file");
    let lines = file.lines();

    let special_cords: Vec<Vec<&i32>> = [].to_vec();

    let mut lineIndex = -1;
    for line in lines {
        lineIndex = lineIndex + 1;
        let mut wordIndex = -1;
        let letters = line.split("");
        for letter in letters {
            wordIndex = wordIndex + 1;
            println!("{}", &letter)
        }
    }
}

fn main() {
    part1();
}

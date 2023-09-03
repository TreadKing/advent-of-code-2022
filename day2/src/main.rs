fn main() {
    let day2_input = include_str!("day2_input.txt");
    let test_input = "A Y\nB X\nC Z";
    // let games: Vec<String> = test_input.split("\n").map(|x| x.to_string()).collect();
 
    let games: Vec<String> = day2_input.split("\n").map(|x| x.to_string()).collect();
    let scores: Vec<i32> = games.iter().map(calc_score).collect();
    let day2a_output: i32 = scores.iter().sum();
    println!("day 2a: {}", day2a_output);

    let scores_2:Vec<i32> = games.iter().map(calc_score_2).collect();
    let day2b_output: i32 = scores_2.iter().sum();
    println!("day 2b {}", day2b_output);
}

fn calc_score(input_str: &String) -> i32 {


    let player1 = input_str.chars().nth(0).unwrap();
    let player2 = input_str.chars().nth(2).unwrap();

    match (player2, player1) {
        ('X', 'A') => 4, // x - draw
        ('X', 'B') => 1, // x - loss
        ('X', 'C') => 7, // x - win
        ('Y', 'A') => 8, // y - win
        ('Y', 'B') => 5, // y - draw
        ('Y', 'C') => 2, // y - loss
        ('Z', 'A') => 3, // z - loss
        ('Z', 'B') => 9, // z - win
        ('Z', 'C') => 6, // z - draw
        _ => {
            println!("Bad Case:");
            println!("\tplayer2: {}", player2);
            println!("\tplayer1 {}", player1);
            0
        }

    }
}

fn calc_score_2(input_str: &String) -> i32 {
    let player1 = input_str.chars().nth(0).unwrap();
    let player2 = input_str.chars().nth(2).unwrap();

    match (player2, player1) {
        ('X', 'A') => 3, // lose to a -> s -> 3
        ('X', 'B') => 1, // lose to b -> r -> 1
        ('X', 'C') => 2, // lose to c -> p -> 2
        ('Y', 'A') => 4, // draw to a -> 4
        ('Y', 'B') => 5, // draw to b -> 5
        ('Y', 'C') => 6, // draw to c -> 6
        ('Z', 'A') => 8, // win to a -> p -> 2 + 6 -> 8
        ('Z', 'B') => 9, // win to b -> s -> 3 + 6 -> 9
        ('Z', 'C') => 7, // win to c -> r -> 1 + 6 -> 7
        _ => {
            println!("Bad Case:");
            println!("\tplayer2: {}", player2);
            println!("\tplayer1 {}", player1);
            0
        }

    }
}


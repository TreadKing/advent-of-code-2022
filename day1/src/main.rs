fn main() {
    let day1_input = include_str!("day1_input.txt");
    let lines: Vec<String> = day1_input
    .split("\n\n").map(|x: &str| x.to_string()).collect();

    let calories: Vec<Vec<i32>> = lines.iter().map(split_lines).collect();

    let mut elfs: Vec<i32> = calories.iter().map(|elf| elf.iter().sum()).collect();
    elfs.sort();
    elfs.reverse();

    let max_cal = elfs.iter().max().unwrap();
    println!("max calories:{}", max_cal);

    let top_3: &i32 = &elfs[0..3].iter().sum();
    println!("top 3 calories: {}", top_3);

}

pub fn split_lines(input_str: &String) -> Vec<i32> {
    let split_lines: Vec<&str> = input_str.split("\n").collect();

    let output: Vec<i32> = split_lines.iter().flat_map(|x| x.parse()).collect();
    output
}

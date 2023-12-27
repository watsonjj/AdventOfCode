use {{crate_name}}::part2::process;

fn main() {
    let input = include_str!("../../input.txt");
    let result = process(input);
    println!("{}", result);
}

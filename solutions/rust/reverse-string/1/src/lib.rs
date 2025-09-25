pub fn reverse(input: &str) -> String {
    let mut output: String = String::new();
    for letter in input.chars().rev()
    {
        output.push(letter);
        println!("Char {}", letter);
    }

    return output;
}

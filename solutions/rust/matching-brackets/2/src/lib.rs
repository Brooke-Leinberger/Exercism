pub fn brackets_are_balanced(string: &str) -> bool {
    let push : [char; 3] = ['[', '(', '{'];
    let pop  : [char; 3] = [']', ')', '}'];

    let chars = string.chars().filter(|ch| push.contains(ch) || pop.contains(ch));
    let mut stack : Vec<char> = Vec::new();

    for ch in chars
    {
        if push.contains(&ch) { stack.push(ch); continue; }
        let left = match stack.pop() { Some(val) => val, None => {return false;}};
        if pop[push.iter().position(|&c| c == left).unwrap()] != ch {return false;}
    }

    stack.is_empty()
}

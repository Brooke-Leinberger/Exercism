pub fn brackets_are_balanced(string: &str) -> bool {
    let push : [char; 3] = ['[', '(', '{'];
    let pop  : [char; 3] = [']', ')', '}'];

    let chars = string.chars().filter(|ch| push.contains(ch) || pop.contains(ch));
    let mut stack : Vec<char> = Vec::new();

    for ch in chars
    {
        if push.contains(&ch) { stack.push(ch); continue; }

        let right = ch;
        let left = stack.pop();
        if left.is_none() { return false; }
        let index : Option<usize> = push.iter().position(|&c| c == left.unwrap());
        match index
        {
            None => { return false; },
            Some(idx) => { if pop[idx] != right {return false;} }
        }
    }

    stack.is_empty()
}

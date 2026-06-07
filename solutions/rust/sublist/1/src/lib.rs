#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn superlist(sub: &[i32], sup: &[i32]) -> Comparison 
{    
    if sup.len() < sub.len() { return Comparison::Unequal; }
    for i in 0..sub.len() { if sub[i] != sup[i] { return Comparison::Unequal; } }
    Comparison::Sublist
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison 
{
    if first_list.len()  == 0 && second_list.len() == 0 { return Comparison::Equal; }
    if first_list.len()  == 0 { return Comparison::Sublist; }
    if second_list.len() == 0 { return Comparison::Superlist; }

    let sub = if first_list.len() < second_list.len() {first_list}  else {second_list};
    let sup = if first_list.len() < second_list.len() {second_list} else {first_list};

    let target = sub[0];
    let starts: Vec<usize> = (0..sup.len())
        .filter_map(|index|
        {
            let result = superlist(sub, &sup[index..]);
            if result != Comparison::Unequal {Some(index)}
            else {None}})
        .collect();

    if starts.len() == 0 { return Comparison::Unequal; }
    if starts[0] == 0 && sub.len() == sup.len() { return Comparison::Equal; }
    if sub == first_list { Comparison::Sublist } else { Comparison::Superlist }
}

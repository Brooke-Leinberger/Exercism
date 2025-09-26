fn pluralize(bottle: u32) -> &'static str {
    match bottle {
        1 => "",
        _ => "s",
    }
}

fn stanza(bottles: u32) -> String {
    let numbers: [&str;11] = ["No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    format!(concat!("{0} green bottle{1} hanging on the wall,\n", 
                    "{0} green bottle{1} hanging on the wall,\n",
                    "And if one green bottle should accidentally fall,\n",
                    "There'll be {2} green bottle{3} hanging on the wall.\n"), 
                    numbers[bottles as usize], pluralize(bottles),
                    numbers[(bottles-1) as usize].to_lowercase(), pluralize(bottles - 1))

}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut stanzas = Vec::new();
    let bottles = start_bottles + 1;
    for i in ((bottles - take_down)..bottles).rev() {
        stanzas.push(stanza(i));
    }

    stanzas.join("\n")
}

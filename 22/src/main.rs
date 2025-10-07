use colored::Colorize;

use std::collections::HashMap;
fn print_board(){
println!("{}", "┌───┬───┬───┬───┬───┐".red());
for i in 0..6 {

println!("├───┼───┼───┼───┼───┤");

}
println!("└───┴───┴───┴───┴───┘");
}
fn compare_words(s1: &str, s2: &str) -> [(char, char); 5]{
    let mut o: [(char, char); 5] = [(' ', ' '); 5];
    let mut a: HashMap<char, i32> = HashMap::new();
    for letter in 'a'..='z' {
        a.insert(letter, 0);
    }
    for l in s1.chars(){
    println!("{}", l);
    }
    let rv: String = String::from("return");
    for l in s1.chars(){
        let h = a.entry(l).or_insert(0); 
        
        
        *h += 1;

    }
    let matches: Vec<_> = s1.chars()
        .zip(s2.chars())
        .enumerate()
        .filter(|&(_, (c1, c2))| c1 == c2)
        .map(|(i, (c, _))| (i, c))
        .collect();

    println!("Matching positions and characters: {:?}", matches);
    for (i, c) in matches { 
        let h = a.entry(c).or_insert(0);
        o[i].0 = c;
        o[i].1 = 'g';
        *h -= 1
    } 
     for (key, value) in o {
        println!("{} → {}", key, value);
    }
    
    for i in 0..5{
    if o[i].0 == ' '{
        o[i].0 = s2.chars().nth(i).unwrap();
        if a[o[i].0] != 0{
            o[i].1 = 'o';
            a[s2.chars().nth(i).unwrap()] -= 1;
        }
        else{
        o[i].0 = s2.chars().nth(i).unwrap();
        o[i].1 = 'b';
        }

        
    }

    }
    
    return o;

}
fn main() {
    print_board();
    compare_words("world", "wordle");
    
}

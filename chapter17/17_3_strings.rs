fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string: String = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    // println!("Used characters: {}", string);

    // The trimmed string is a slice to the original string,
    // hence no new allocation is performed
    let trimmed_str: &str = string.trim_matches(&[' ', ','] as &[char]);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice: String = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

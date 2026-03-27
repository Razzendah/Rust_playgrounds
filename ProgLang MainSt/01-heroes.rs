fn main() {
    let mut heroes = vec!["Iron Man", "Thor", "Hulk"];

    // This would cause a panic in many languages
    // but Rust handles it gracefully
    let hero = heroes.get(10);

    match hero {
        Some(name) => println!("Found hero: {}", name),
        None => println!("No hero at that index - mission aborted!"),
    }

    // Safe array access
    if let Some(first_hero) = heroes.get(0) {
        println!("Our lead hero: {}", first_hero);
    }

    heroes.push("Captain Marvel");
    println!("Team size: {}", heroes.len());
}
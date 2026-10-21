const FACTORS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let mut result = String::with_capacity(15);

    for &(factor, sound) in &FACTORS {
        if n.is_multiple_of(factor) {
            result.push_str(sound);
        }
    }
    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}

const FACTORS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    Some(
        FACTORS
            .iter()
            .filter(|(factor, _)| n.is_multiple_of(*factor))
            .map(|(_, sound)| *sound)
            .collect::<String>(),
    )
    .filter(|s| !s.is_empty())
    .unwrap_or_else(|| n.to_string())
}

pub fn shannon_entropy_bytes(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }

    let len = data.len() as f64;

    counts
        .into_iter()
        .filter(|c| *c != 0)
        .map(|p| (p as f64) / len)
        .fold(0.0, |acc, p| acc - (p * p.log2()))
}

pub fn shannon_entropy_str(s: &str) -> f64 {
    shannon_entropy_bytes(s.as_bytes())
}

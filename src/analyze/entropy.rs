pub fn shannon_entropy_bytes(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }

    let len = data.len() as f64;
    let mut h = 0.0;

    for &c in &counts {
        if c == 0 {
            continue;
        }
        let p = (c as f64) / len;
        h -= p * p.log2();
    }

    h
}

pub fn shannon_entropy_str(s: &str) -> f64 {
    shannon_entropy_bytes(s.as_bytes())
}


use std::collections::HashMap;
use std::fmt;

struct BasicStats {
    median: f32,
    mode: i32,
}

fn calculate_stats(ints: &mut Vec<i32>) -> BasicStats {
    let mut stats = BasicStats{
        mode: -1,
        median: -1.0,
    };

    let mut map = HashMap::new();
    let mut sum: f32 = 0.0;

    ints.sort();

    for x in ints.iter() {
        sum += *x as f32;
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    if !ints.is_empty() {
        stats.median = sum / (ints.len() as f32);
    }

    // Calcular la moda. Obtengo el valor de frecuencia obteniendo el primer elemento
    // del vector.
    let mut max_freq = map[&ints[0]];
    stats.mode = max_freq;

    for (&num, &freq) in &map {
        if freq > max_freq {
            stats.mode = *num;
            max_freq = freq;
        }
    }

    stats
}

impl fmt::Display for BasicStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Median: {}, Mode: {}", self.median, self.mode)
    }
}

fn main() {
    let mut ints = vec![1, 2, 3, 4, 2];
    println!("ints=({:?})", ints);

    let result = calculate_stats(&mut ints);
    println!("Value = {}", result);
}

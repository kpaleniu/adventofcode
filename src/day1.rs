pub fn increments(measurements: &[u64]) -> u64 {
    let mut increments = 0;
    let mut sum: Option<u64> = None;
    for next in measurements {
        if let Some(prev) = sum {
            if next > &prev {
                increments += 1;
            }
        }
        sum = Some(*next);
    }
    increments
}

pub fn windowed(measurements: &[u64]) -> u64 {
    let mut increments = 0;
    let mut sum: Option<u64> = None;
    for i in 0..measurements.len() {
        if i + 3 > measurements.len() {
            break;
        }
        let next = measurements[i..(i + 3)].iter().sum();
        if let Some(prev) = sum {
            if next > prev {
                increments += 1;
            }
        }
        sum = Some(next);
    }
    increments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increments() {
        let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let value = increments(&measurements);
        assert_eq!(value, 7);
    }

    #[test]
    fn test_windowed() {
        let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let value = windowed(&measurements);
        assert_eq!(value, 5);
    }
}

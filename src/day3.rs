fn parse(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    input
        .lines()
        .map(|val| u64::from_str_radix(val, 2))
        .collect()
}

fn bit_counts(nums: Vec<u64>, radix: usize) -> Vec<(u64, u64)> {
    let mut counts = vec![(0, 0); radix];
    for n in nums {
        for i in 0..counts.len() {
            let mut c = &mut counts[i];
            let v = (n >> (radix - 1 - i)) & 1;
            c.0 += v ^ 1;
            c.1 += v & 1;
        }
    }
    counts
}

pub fn part1(input: &str, radix: usize) -> Result<(u64, u64), std::num::ParseIntError> {
    let nums = parse(input)?;
    let counts = bit_counts(nums, radix);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..counts.len() {
        let (zeros, ones) = counts[i];
        gamma += ((ones > zeros) as u64) << (radix - 1 - i);
        epsilon += ((zeros > ones) as u64) << (radix - 1 - i);
    }

    Ok((gamma, epsilon))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        if let Ok((gamma, epsilon)) = part1(input, 5) {
            assert_eq!(gamma, 22);
            assert_eq!(epsilon, 9);
            assert_eq!(gamma * epsilon, 198);
        }
    }
}

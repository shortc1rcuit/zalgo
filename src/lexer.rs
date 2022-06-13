use itertools::Itertools;

pub mod cluster;
use cluster::*;

pub fn lex_text(base_code: &str) -> Vec<Cluster> {
    let centre_chars: Vec<usize> = get_centre_chars_loc(base_code);

    println!("{:#?}", centre_chars);
    todo!();
}

fn pair(unpaired: Vec<u32>) -> Vec<(u32, u32)>{
    let mut paired = Vec::new();

    for (a, b) in unpaired.into_iter().tuple_windows() {
        paired.push((a, b));
    }

    paired
}

fn get_centre_chars_loc(base_code: &str) -> Vec<usize> {
    base_code
        .chars()
        .map(|x| x as u32)
        .enumerate()
        .filter(|x| x.1 < 0x300 || x.1 > 0x36F)
        .map(|x| x.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centre_chars_loc() {
        let base_code = "A\u{0310}\u{030F}\u{0306}B\u{031D}";

        assert_eq!(get_centre_chars_loc(base_code), vec![0, 4]);
    }

    #[test]
    fn pairs() {
        let unpaired = vec![1, 2, 3, 4, 5];
        let paired = vec![(1, 2), (2, 3), (3, 4), (4, 5)];

        assert_eq!(pair(unpaired), paired);
    }
}

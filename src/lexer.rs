use itertools::Itertools;

pub mod cluster;
use cluster::*;

pub fn lex_text(base_code: String) -> Result<Vec<Cluster>, &'static str> {
    let base_code = strip_whitespace(&base_code);

    let cluster_starts = get_centre_chars_loc(&base_code);
    if cluster_starts.is_empty() {
        return Err("No base characters");
    }
    let cluster_starts = pair(cluster_starts);

    let unclustered = get_cluster_text(base_code, cluster_starts);

    Ok(text_to_cluster(unclustered))
}

fn strip_whitespace(a: &str) -> String {
    a.chars().filter(|x| !x.is_whitespace()).collect()
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

fn pair(unpaired: Vec<usize>) -> Vec<(usize, Option<usize>)> {
    let mut paired = Vec::new();
    //Unwrap is fine as I've checked that unpaired has at least one value in it
    let last = *unpaired.last().unwrap();

    for (a, b) in unpaired.into_iter().tuple_windows() {
        paired.push((a, Some(b)));
    }

    paired.push((last, None));
    paired
}

fn get_cluster_text(base_code: String, cluster_starts: Vec<(usize, Option<usize>)>) -> Vec<String> {
    let mut unclustered: Vec<String> = Vec::new();

    //Runs through the pairs and gets the text between them
    //If this is at the end of the text (signified by a None)
    //Then just return the rest of the text
    for (a, b) in cluster_starts {
        let a = a + 1;
        match b {
            Some(x) => unclustered.push(base_code.chars().skip(a).take(x - a).collect()),
            None => unclustered.push(base_code.chars().skip(a).collect()),
        };
    }

    //If two non-diacritic characters are next to each other
    //cluster_text will have an empty Vec
    //This filters those out
    unclustered.into_iter().filter(|x| !x.is_empty()).collect()
}

fn text_to_cluster(unclustered: Vec<String>) -> Vec<Cluster> {
    unclustered
        .into_iter()
        .map(|x| x.chars().into_iter().map(|a| (a as u32) - 0x300).collect())
        .map(Cluster::new)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_whitespace_test() {
        let a = "a b
        c";

        assert_eq!(strip_whitespace(a), "abc".to_string());
    }

    #[test]
    fn centre_chars_loc_test() {
        let base_code = "A\u{0310}\u{030F}\u{0306}B\u{031D}".to_string();

        assert_eq!(get_centre_chars_loc(&base_code), vec![0, 4]);
    }

    #[test]
    fn pairs_test() {
        let unpaired = vec![1, 2, 3, 4, 5];
        let paired = vec![
            (1, Some(2)),
            (2, Some(3)),
            (3, Some(4)),
            (4, Some(5)),
            (5, None),
        ];

        assert_eq!(pair(unpaired), paired);
    }

    #[test]
    fn single_pair_test() {
        let unpaired = vec![1];
        let paired = vec![(1, None)];

        assert_eq!(pair(unpaired), paired);
    }

    #[test]
    fn cluster_text_test() {
        let base_code = "A\u{0310}\u{030F}\u{0306}BC\u{031D}".to_string();
        let cluster_starts = vec![(0, Some(4)), (4, Some(5)), (5, None)];
        let unclustered = vec!["\u{0310}\u{030F}\u{0306}", "\u{031D}"];

        assert_eq!(get_cluster_text(base_code, cluster_starts), unclustered);
    }

    #[test]
    fn single_cluster_text_test() {
        let base_code = "A\u{0310}\u{030F}\u{0306}".to_string();
        let cluster_starts = vec![(0, None)];
        let unclustered = vec!["\u{0310}\u{030F}\u{0306}"];

        assert_eq!(get_cluster_text(base_code, cluster_starts), unclustered);
    }

    #[test]
    fn text_to_cluster_test() {
        let unclustered = vec![
            "\u{0310}\u{030F}\u{0306}".to_string(),
            "\u{031D}".to_string(),
        ];
        let clustered = vec![
            Cluster {
                top: vec![6, 0xF, 0x10],
                bottom: vec![],
            },
            Cluster {
                top: vec![],
                bottom: vec![0x1D],
            },
        ];

        assert_eq!(text_to_cluster(unclustered), clustered);
    }
}

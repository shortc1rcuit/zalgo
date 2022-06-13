pub mod cluster;
use cluster::*;

pub fn lex_text(base_code: &str) -> Vec<Cluster>{
    let centre_chars: Vec<usize> = get_centre_chars_loc(base_code);

    println!("{:#?}", centre_chars);
    todo!();
}

fn get_centre_chars_loc(base_code: &str) -> Vec<usize>{
    base_code.chars()
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
}

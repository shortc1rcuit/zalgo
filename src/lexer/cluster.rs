pub mod token;
use token::*;

#[derive(Debug, PartialEq)]
pub struct Cluster {
    pub top: Vec<TopSet>,
    pub bottom: Vec<BottomSet>,
}

impl Cluster {
    pub fn new(unclustered: Vec<u32>) -> Cluster {
        let mut top = Vec::new();
        let mut bottom = Vec::new();

        for c in unclustered {
            if c <= 0xF {
                top.push(TopSet::Number(c as i32));
            } else if c == 0x10 {
                top.push(TopSet::Push);
            } else if c == 0x11 {
                top.push(TopSet::Pop);
            } else if c == 0x1C {
                bottom.push(BottomSet::If);
            } else if c == 0x1D {
                bottom.push(BottomSet::Print);
            } else if c == 0x48 {
                bottom.push(BottomSet::Dup);
            } else if c == 0x1F {
                bottom.push(BottomSet::Add);
            } else if c == 0x20 {
                bottom.push(BottomSet::Sub);
            } else if c == 0x53 {
                bottom.push(BottomSet::Mul);
            } else if c == 0x21 {
                bottom.push(BottomSet::Div);
            } else if c == 0x22 {
                bottom.push(BottomSet::Mod);
            }
        }

        top.reverse();

        Cluster { top, bottom }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cluster() {
        let unclustered = vec![0x1D, 15, 6, 0x1D, 7, 6];
        let clustered = Cluster {
            top: vec![
                TopSet::Number(6),
                TopSet::Number(7),
                TopSet::Number(6),
                TopSet::Number(15),
            ],
            bottom: vec![BottomSet::Print, BottomSet::Print],
        };

        assert_eq!(Cluster::new(unclustered), clustered);
    }
}

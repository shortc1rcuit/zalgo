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
                top.push(TopSet::Number(c));
            } else if c == 0x10 {
                top.push(TopSet::Push);
            } else if c == 0x11 {
                top.push(TopSet::Pop);
            } else if c == 0x25 {
                bottom.push(BottomSet::If);
            } else if c == 0x1D {
                bottom.push(BottomSet::Print);
            } else if c == 0x1E {
                bottom.push(BottomSet::Input);
            } else if c == 0x48 {
                bottom.push(BottomSet::Dup);
            } else if c == 0x4D {
                bottom.push(BottomSet::Jump);
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
            } else if c == 0x2D {
                bottom.push(BottomSet::And);
            } else if c == 0x2C {
                bottom.push(BottomSet::Or);
            } else if c == 0x49 {
                bottom.push(BottomSet::Not);
            } else if c == 0x1C {
                bottom.push(BottomSet::Bsl);
            } else if c == 0x39 {
                bottom.push(BottomSet::Bsr);
            } else if c == 0x33 {
                bottom.push(BottomSet::Equal);
            } else if c == 0x55 {
                bottom.push(BottomSet::Greater);
            } else if c == 0x54 {
                bottom.push(BottomSet::Less);
            } else if c == 0x19 {
                bottom.push(BottomSet::CycleUp);
            } else if c == 0x18 {
                bottom.push(BottomSet::CycleDown);
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

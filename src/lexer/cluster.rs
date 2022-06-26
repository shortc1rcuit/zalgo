const TOP_SET: [u32; 18] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];

const BOTTOM_SET: [u32; 3] = [0x1C, 0x1D, 0x48];

#[derive(Debug, PartialEq)]
pub struct Cluster {
    pub top: Vec<u32>,
    pub bottom: Vec<u32>,
}

impl Cluster {
    pub fn new(unclustered: Vec<u32>) -> Cluster {
        let mut top = Vec::new();
        let mut bottom = Vec::new();

        for c in unclustered {
            if TOP_SET.contains(&c) {
                top.push(c);
            } else if BOTTOM_SET.contains(&c) {
                bottom.push(c);
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
            top: vec![6, 7, 6, 15],
            bottom: vec![0x1D, 0x1D],
        };

        assert_eq!(Cluster::new(unclustered), clustered);
    }
}

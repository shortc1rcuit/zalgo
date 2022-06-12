const TOP_SET: [u32;17] = [0, 1, 2, 3, 4, 5, 6, 7, 8,
                           9, 10, 11, 12, 13, 14, 15, 16];

const BOTTOM_SET: [u32;1] = [0x1D];

pub struct Cluster {
    pub top: Vec<u32>,
    pub bottom: Vec<u32>
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

        Cluster {
            top,
            bottom,
        }
    }
}

use std::cmp::Ordering;

fn main() {
    let mut groups = vec![];
    let mut group = vec![];
    for arg in std::env::args().skip(1) {
        if arg == "--" {
            groups.push(group);
            group = vec![]
        } else {
            group.push(arg);
        }
    }
    groups.push(group);
    assert!(groups.iter().all(|g| g.len() > 0));
    assert_eq!(groups.len(), 2);

    let sames: Vec<SparseBytes> = groups
        .iter()
        .map(|group| {
            let (first, rest) = group.as_slice().split_first().unwrap();
            let mut same = SparseBytes::new(&std::fs::read(first).unwrap());
            for name in rest {
                same.intersect(&std::fs::read(name).unwrap());
            }
            same
        })
        .collect();

    for pos in diff(&sames[0], &sames[1]) {
        if pos >= 0x1000 {
            break;
        }
        println!(
            "{pos:#10x}: {:02x} {:02x}",
            sames[0].get(pos).unwrap(),
            sames[1].get(pos).unwrap()
        )
    }
}

fn diff(s1: &SparseBytes, s2: &SparseBytes) -> Vec<usize> {
    let mut i1 = 0;
    let mut i2 = 0;

    let mut d = vec![];

    loop {
        if i1 >= s1.0.len() {
            //d.extend((i2..s2.b.len()).map(|i| s2.b[i].0));
            break;
        }
        if i2 >= s2.0.len() {
            //d.extend((i1..s1.b.len()).map(|i| s1.b[i].0));
            break;
        }
        let b1 = &s1.0[i1];
        let b2 = &s2.0[i2];
        match b1.0.cmp(&b2.0) {
            Ordering::Less => {
                //d.push(b1.0);
                i1 += 1;
            }
            Ordering::Greater => {
                //d.push(b2.0);
                i2 += 1;
            }
            Ordering::Equal => {
                if b1.1 != b2.1 {
                    d.push(b1.0);
                }
                i1 += 1;
                i2 += 1;
            }
        }
    }

    d
}

struct SparseBytes(Vec<(usize, u8)>);

impl SparseBytes {
    fn new(v: &[u8]) -> Self {
        Self(v.iter().copied().enumerate().collect())
    }

    fn get(&self, p: usize) -> Option<u8> {
        self.0
            .binary_search_by_key(&p, |&(pos, _b)| pos)
            .map(|i| self.0[i].1)
            .ok()
    }

    fn intersect(&mut self, new_v: &[u8]) {
        self.0
            .retain(|&(pos, b)| new_v.get(pos).is_some_and(|&new_b| new_b == b))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let v1 = vec![0, 17, 2, 3];
        let v2 = vec![1, 17, 2, 4];

        let mut same = SparseBytes::new(&v1);

        for (i, b1) in v1.iter().copied().enumerate() {
            assert_eq!(same.get(i), Some(b1));
        }

        same.intersect(&v2);
        assert_eq!(same.get(1usize), Some(17u8));
        assert_eq!(same.0, vec![(1, 17u8), (2, 2u8)]);
    }
}

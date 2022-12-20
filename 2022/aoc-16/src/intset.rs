#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct IntSet {
    pub v: usize,
}

impl IntSet {
    pub fn new(vs: &Vec<u8>) -> Self {
        let mut set = Self { v: 0 };
        for v in vs {
            set.insert(*v)
        }
        set
    }

    pub fn new_with(v: usize) -> Self {
        Self {v}
    }

    pub fn insert(&mut self, id: u8) {
        self.v |= 1 << id
    }

    pub fn contains(&self, id: u8) -> bool {
        ((self.v >> id) & 1) > 0
    }

    pub fn remove(&mut self, id: u8) {
        self.v &= !(1 << id)
    }

    pub fn items(&self) -> Vec<u8> {
        let mut v = self.v;
        let mut i = 0;
        let mut ids = vec![];
        while v > 0 {
            if v % 2 == 1 {
                ids.push(i);
            }
            i += 1;
            v >>= 1
        }
        ids
    }

    pub fn len(&self) -> usize {
        let mut v = self.v;
        let mut l = 0;
        while v > 0 {
            if v % 2 == 1 {l += 1};
            v >>= 1;
        }
        l
    }

    pub fn is_empty(&self) -> bool {
        self.v == 0
    }
}

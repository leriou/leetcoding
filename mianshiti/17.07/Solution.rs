impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut u = UnionFind::with_capacity(m.len());
        for i in 0..m.len() {
            for j in 0..i{
                if m[i][j] == 1 {
                    u.union(i,j);
                }
            }
        }
        u.count() as i32
    }
}

pub struct UnionFind{
    parents: Vec<usize>,
    size: Vec<usize>,
    count: usize
}

impl UnionFind {

    pub fn with_capacity(n: usize) -> Self {
        let mut v = vec![0;n];
        for i in 0..n {
            v[i] = i;
        }
        UnionFind {
            parents: v,
            size: vec![1;n],
            count: n
        }
    }

    pub fn find(&mut self, mut p:usize) -> usize {
        while self.parents[p] != p {
            self.parents[p] = self.parents[self.parents[p]];
            p = self.parents[p];
        }
        p
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i != j {
            self.count -= 1;
            if self.size[i] > self.size[j] {
                self.parents[j] = i;
                self.size[i] += self.size[j];
            } else {
                self.parents[i] = j;
                self.size[j] += self.size[i];
            }
        }
    }

    pub fn count(&self) -> usize {
      self.count
    }
}
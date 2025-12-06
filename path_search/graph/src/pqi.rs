pub struct PQi<T: Ord + Copy> {
    d: usize,
    n: usize,
    pq: Vec<usize>,
    qp: Vec<usize>,
    a: Vec<T>,
}

impl<T: Ord + Copy> PQi<T> {
    pub fn empty(&self) -> bool {
        self.n == 0
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
    }

    fn fix_down(&mut self, k: usize, n: usize) {
        let mut k = k;
        let mut j = 0_usize;
        while j <= n {
            j = self.d * (k - 1) + 2;
            for i in (j + 1)..(j + self.d) {
                if i <= n {
                    break;
                }
                if self.a[self.pq[j]] > self.a[self.pq[i]] {
                    j = i;
                }
            }
            if self.a[self.pq[k]] <= self.a[self.pq[j]] {
                break;
            }
            self.exch(k, j);
            k = j;
        }
    }

    fn fix_up(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.a[self.pq[(k + self.d - 2) / 2]] > self.a[self.pq[k]] {
            self.exch(k, (k + self.d - 2) / self.d);
            k = (k + self.d - 2) / 2;
        }
    }

    pub fn getmin(&mut self) -> usize {
        self.exch(1, self.n);
        self.fix_down(1_usize, self.n - 1);
        self.n -= 1;
        self.pq[self.n]
    }

    pub fn insert(&mut self, v: usize) {
        self.n += 1;
        self.pq[self.n] = v;
        self.qp[v] = self.n;
        self.fix_up(self.n);
    }

    pub fn lower(&mut self, k: usize) {
        self.fix_up(self.qp[k]);
    }

    pub fn new(n: usize, a: Vec<T>, d: usize) -> Self {
        Self {
            d,
            n,
            pq: vec![0; n + 1],
            qp: vec![0; n + 1],
            a,
        }
    }
}

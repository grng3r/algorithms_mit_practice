pub struct Wqupc{
    id: Vec<usize>,//list of node id's
    sz : Vec<usize>,//number of nodes in tree root -> sz
}

impl Wqupc{
    pub fn new(n: usize) -> Self{
        Wqupc{
            id: (0..n).map(|x| x).collect(),
            sz: vec![0; n]
        }
    }
    pub fn count(&self) -> usize{
        self.id.len()
    }
    
    pub fn connected(&mut self, p: usize, q:usize) -> bool{
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p:usize, q: usize){
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return
        }
        if self.sz[i] < self.sz[j]{
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else{
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }

    fn root(&mut self, i: usize) -> usize{
        let mut root = i;
        while root != self.id[i]{
            self.id[root] = self.id[self.id[root]];
            root = self.id[i];
        }
        root
    }
}
//TODO tests

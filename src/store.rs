use crate::nodes::*;
use crate::utils::*;
use crate::*;

pub struct Store {
    ints: [Vec<Box<dyn Program<Int>>>; MAX_SIZE + 1],
    int_set: HashSet<Vec<Int>>,
    strs: [Vec<Box<dyn Program<Str>>>; MAX_SIZE + 1],
    str_set: HashSet<Vec<Str>>,
}

impl ProgramStore<Int> for Store {
    fn get(&self, index: Index<Int>) -> Option<&Box<dyn Program<Int>>> {
        if self.has(index) {
            Some(&self.ints[index.size][index.idx])
        } else {
            None
        }
    }

    fn put(&mut self, program: Box<dyn Program<Int>>) -> Option<Index<Int>> {
        let values = program.values();

        if self.int_set.contains(values) {
            None
        } else {
            self.int_set.insert(values.to_vec());
            let size = program.size() as usize;
            self.ints[size].push(program);
            Some(Index::new(size, self.ints[size].len() - 1))
        }
    }

    fn get_unchecked(&self, index: Index<Int>) -> &Box<dyn Program<Int>> {
        &self.ints[index.size][index.idx]
    }

    fn has(&self, index: Index<Int>) -> bool {
        index.size <= MAX_SIZE && index.idx < self.ints[index.size].len()
    }
}

impl ProgramStore<Str> for Store {
    fn get(&self, index: Index<Str>) -> Option<&Box<dyn Program<Str>>> {
        if self.has(index) {
            Some(&self.strs[index.size][index.idx])
        } else {
            None
        }
    }

    fn put(&mut self, program: Box<dyn Program<Str>>) -> Option<Index<Str>> {
        let values = program.values();

        if self.str_set.contains(values) {
            None
        } else {
            self.str_set.insert(values.to_vec());
            let size = program.size() as usize;
            self.strs[size].push(program);
            Some(Index::new(size, self.strs[size].len() - 1))
        }
    }

    fn get_unchecked(&self, index: Index<Str>) -> &Box<dyn Program<Str>> {
        &self.strs[index.size][index.idx]
    }

    fn has(&self, index: Index<Str>) -> bool {
        index.size < MAX_SIZE && index.idx < self.strs[index.size].len()
    }
}

impl Store {
    pub fn new(task: Task) -> Self {
        let int_vars = task
            .ints()
            .into_iter()
            // TODO Why is the explicit cast required?!
            .map(|(name, values)| Variable::new(name, values) as Box<dyn Program<Int>>)
            .collect();
        let str_vars = task
            .strs()
            .into_iter()
            .map(|(name, values)| Variable::new(name, values) as Box<dyn Program<Str>>)
            .collect();

        let ints = [int_vars, Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let strs = [str_vars, Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        Self {
            ints,
            int_set: HashSet::new(),
            strs,
            str_set: HashSet::new(),
        }
    }
}

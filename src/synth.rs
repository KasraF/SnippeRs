use crate::{nodes::binary::*, nodes::unary::*, predicates::ValuePredicate, store::*, utils::*, *};

enum SynthEnum {
    Int(usize, Box<dyn Enumerator<Int>>),
    Str(usize, Box<dyn Enumerator<Str>>),
    IntList(usize, Box<dyn Enumerator<IntList>>),
    StrList(usize, Box<dyn Enumerator<StrList>>),
}

impl SynthEnum {
    fn has_next(&self, store: &Store) -> bool {
        match self {
            SynthEnum::Int(_, e) => e.has_next(store),
            SynthEnum::Str(_, e) => e.has_next(store),
            SynthEnum::IntList(_, e) => e.has_next(store),
            SynthEnum::StrList(_, e) => e.has_next(store),
        }
    }
}

pub struct Synthesizer {
    store: Store,
    int_enums: Vec<Builder<Int>>,
    str_enums: Vec<Builder<Str>>,
    intlist_enums: Vec<Builder<IntList>>,
    strlist_enums: Vec<Builder<StrList>>,
    curr_enum: SynthEnum,
    pred: Box<dyn Predicate>,
    size: usize,
}

impl Synthesizer {
    pub fn new(task: Task) -> Self {
        let int_enums = vec![
            // UnaryEnum::builder(unary_true_validator, to_string_value, to_string_code),
            BinEnum::builder(bin_true_validator, add_value, add_code),
            BinEnum::builder(bin_true_validator, sub_value, sub_code),
            UnaryEnum::builder(unary_true_validator, list_len_value::<Int>, list_len_code),
            UnaryEnum::builder(unary_true_validator, list_len_value::<Str>, list_len_code),
        ];

        let str_enums = vec![UnaryEnum::builder(
            unary_true_validator,
            to_string_value,
            to_string_code,
        )];

        let intlist_enums: Vec<Builder<IntList>> = vec![BinEnum::builder(
            bin_true_validator,
            list_concat_value,
            list_concat_code,
        )];

        let strlist_enums = vec![
            UnaryEnum::builder(unary_true_validator, string_split_value, string_split_code),
            BinEnum::builder(bin_true_validator, list_concat_value, list_concat_code),
        ];

        let size = 1;

        let curr_enum = SynthEnum::Int(0, int_enums[0](size));

        let pred = Box::new(ValuePredicate::new(&task));

        Self {
            store: Store::new(task),
            int_enums,
            str_enums,
            intlist_enums,
            strlist_enums,
            curr_enum,
            pred,
            size,
        }
    }
}

impl Iterator for Synthesizer {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.curr_enum.has_next(&self.store) {
            // Get the next enum
            self.curr_enum = match &self.curr_enum {
                SynthEnum::Int(i, _) => {
                    if self.int_enums.len() > i + 1 {
                        SynthEnum::Int(i + 1, (self.int_enums[*i])(self.size))
                    } else {
                        SynthEnum::Str(0, (self.str_enums[0])(self.size))
                    }
                }
                SynthEnum::Str(i, _) => {
                    if self.str_enums.len() > i + 1 {
                        SynthEnum::Str(i + 1, (self.str_enums[*i])(self.size))
                    } else {
                        SynthEnum::IntList(0, (self.intlist_enums[0])(self.size))
                    }
                }
                SynthEnum::IntList(i, _) => {
                    if self.intlist_enums.len() > i + 1 {
                        SynthEnum::IntList(i + 1, (self.intlist_enums[*i])(self.size))
                    } else {
                        SynthEnum::StrList(0, (self.strlist_enums[0])(self.size))
                    }
                }
                SynthEnum::StrList(i, _) => {
                    if self.strlist_enums.len() > i + 1 {
                        SynthEnum::StrList(i + 1, (self.strlist_enums[*i])(self.size))
                    } else {
                        // We need to go to the next size!
                        if self.size == MAX_SIZE {
                            return None;
                        }
                        self.size += 1;
                        SynthEnum::Int(0, (self.int_enums[0])(self.size))
                    }
                }
            }
        }

        // We are at a valid enumerator, so just enumerate!
        // TODO There has to be a better way to do this. macros?
        match &mut self.curr_enum {
            SynthEnum::Int(_, e) => {
                let program = e.next(&self.store).unwrap();
                let idx = self.store.put(program);
                if let Some(idx) = idx {
                    let program = self.store.get_unchecked(idx);
                    let code = program.code(&self.store);
                    dbg!(&code);

                    if self.pred.matches(program.as_ref()) {
                        return Some(Some(code));
                    }
                }
            }
            SynthEnum::Str(_, e) => {
                let program = e.next(&self.store).unwrap();
                let idx = self.store.put(program);
                if let Some(idx) = idx {
                    let program = self.store.get_unchecked(idx);
                    let code = program.code(&self.store);
                    dbg!(&code);

                    if self.pred.matches(program.as_ref()) {
                        return Some(Some(code));
                    }
                }
            }
            SynthEnum::IntList(_, e) => {
                let program = e.next(&self.store).unwrap();
                let idx = self.store.put(program);
                if let Some(idx) = idx {
                    let program = self.store.get_unchecked(idx);
                    let code = program.code(&self.store);
                    dbg!(&code);

                    if self.pred.matches(program.as_ref()) {
                        return Some(Some(code));
                    }
                }
            }
            SynthEnum::StrList(_, e) => {
                let program = e.next(&self.store).unwrap();
                let idx = self.store.put(program);
                if let Some(idx) = idx {
                    let program = self.store.get_unchecked(idx);
                    let code = program.code(&self.store);
                    dbg!(&code);

                    if self.pred.matches(program.as_ref()) {
                        return Some(Some(code));
                    }
                }
            }
        }

        Some(None)
    }
}

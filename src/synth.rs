use crate::{nodes::*, predicates::ValuePredicate, store::*, utils::*, *};

enum SynthEnum {
    Int(usize, Box<dyn Enumerator<Int>>),
    Str(usize, Box<dyn Enumerator<Str>>),
}

pub struct Synthesizer {
    store: Store,
    int_enums: Vec<Builder<Int>>,
    str_enums: Vec<Builder<Str>>,
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
        ];

        let str_enums = vec![UnaryEnum::builder(
            unary_true_validator,
            to_string_value,
            to_string_code,
        )];

        let size = 1;

        let curr_enum = SynthEnum::Int(0, int_enums[0](size));

        let pred = Box::new(ValuePredicate::new(&task));

        Self {
            store: Store::new(task),
            int_enums,
            str_enums,
            curr_enum,
            pred,
            size,
        }
    }
}

impl Iterator for Synthesizer {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while match &self.curr_enum {
            SynthEnum::Int(_, e) => !e.has_next(&self.store),
            SynthEnum::Str(_, e) => !e.has_next(&self.store),
        } {
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
        }

        Some(None)
    }
}

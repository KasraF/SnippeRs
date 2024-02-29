use crate::store::Bank;
use crate::synth::Enumerator;
use crate::{utils::*, Level};
use crate::{BinBuilder, UniBuilder};

mod int;
mod str;

pub type Vocab = Vec<Builder>;

pub enum ConstVal {
    Int(&'static str, Int),
    Str(&'static str, Str),
}

pub fn constants() -> Vec<ConstVal> {
    vec![
        ConstVal::Int("0", 0),
        ConstVal::Int("1", 1),
        ConstVal::Str("\"\"", "".to_string()),
        ConstVal::Str("\" \"", " ".to_string()),
    ]
}

pub fn vocab() -> Vocab {
    vec![
        UniBuilder::new(&str::len_eval, &str::len_code).into(),
        BinBuilder::new(&str::deref_eval, &str::deref_code).into(),
        UniBuilder::new(&int::minus_eval, &int::minus_code).into(),
        BinBuilder::new(&int::sum_eval, &int::sum_code).into(),
        BinBuilder::new(&int::sub_eval, &int::sub_code).into(),
        UniBuilder::new(&int::inc_eval, &int::inc_code).into(),
    ]
}

pub enum Builder {
    UnaryIntInt(UniBuilder<Int, Int>),
    UnaryIntStr(UniBuilder<Int, Str>),
    UnaryStrInt(UniBuilder<Str, Int>),
    UnaryStrStr(UniBuilder<Str, Str>),
    BinaryIntIntInt(BinBuilder<Int, Int, Int>),
    BinaryIntIntStr(BinBuilder<Int, Int, Str>),
    BinaryIntStrInt(BinBuilder<Int, Str, Int>),
    BinaryIntStrStr(BinBuilder<Int, Str, Str>),
    BinaryStrIntInt(BinBuilder<Str, Int, Int>),
    BinaryStrIntStr(BinBuilder<Str, Int, Str>),
    BinaryStrStrInt(BinBuilder<Str, Str, Int>),
    BinaryStrStrStr(BinBuilder<Str, Str, Str>),
}

impl From<UniBuilder<Str, Int>> for Builder {
    fn from(value: UniBuilder<Str, Int>) -> Self {
        Builder::UnaryStrInt(value)
    }
}

impl From<UniBuilder<Int, Int>> for Builder {
    fn from(value: UniBuilder<Int, Int>) -> Self {
        Builder::UnaryIntInt(value)
    }
}

impl From<BinBuilder<Int, Int, Int>> for Builder {
    fn from(value: BinBuilder<Int, Int, Int>) -> Self {
        Builder::BinaryIntIntInt(value)
    }
}

impl From<BinBuilder<Str, Int, Str>> for Builder {
    fn from(value: BinBuilder<Str, Int, Str>) -> Self {
        Builder::BinaryStrIntStr(value)
    }
}

impl Builder {
    pub fn enumerator(&self, level: Level, store: &Bank) -> Box<dyn Enumerator> {
        let max_idx = store.curr_max();
        match &self {
            Builder::UnaryIntInt(builder) => builder.into_enum(level, max_idx),
            Builder::UnaryIntStr(builder) => builder.into_enum(level, max_idx),
            Builder::UnaryStrInt(builder) => builder.into_enum(level, max_idx),
            Builder::UnaryStrStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntIntInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntIntStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntStrInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryIntStrStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrIntInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrIntStr(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrStrInt(builder) => builder.into_enum(level, max_idx),
            Builder::BinaryStrStrStr(builder) => builder.into_enum(level, max_idx),
        }
    }
}

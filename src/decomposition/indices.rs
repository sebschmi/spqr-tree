use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use num_traits::{Bounded, PrimInt};
use optional_numeric_index::implement_generic_index;

pub trait GraphIndexInteger:
    PrimInt + Bounded + Hash + Debug + Display + From<u8> + TryFrom<usize> + TryInto<usize>
{
}

impl<T: PrimInt + Bounded + Hash + Debug + Display + From<u8> + TryFrom<usize> + TryInto<usize>>
    GraphIndexInteger for T
{
}

pub trait GraphIndex: Copy + Eq + Ord + Hash + Debug + Display + From<usize> + Into<usize> {}

impl<T: Copy + Eq + Ord + Hash + Debug + Display + From<usize> + Into<usize>> GraphIndex for T {}

implement_generic_index!(pub ComponentIndex, pub OptionalComponentIndex);
implement_generic_index!(pub BlockIndex, pub OptionalBlockIndex);
implement_generic_index!(pub CutNodeIndex, pub OptionalCutNodeIndex);
implement_generic_index!(pub SPQRNodeIndex, pub OptionalSPQRNodeIndex);
implement_generic_index!(pub SPQREdgeIndex, pub OptionalSPQREdgeIndex);

#![allow(non_camel_case_types)]

use pauli_tracker::pauli::{
    Pauli,
    PauliDense,
    PauliEnum,
    PauliStack,
    PauliTuple,
};

use crate::boolean_vector::{
    BitVec,
    Vec_b,
};

pub type PauliStack_vb = PauliStack<Vec_b>;
pub type PauliStack_bv = PauliStack<BitVec>;

impl_api::pauli!(PauliDense, pauli_dense_);
impl_api::pauli!(PauliEnum, pauli_enum_);
impl_api::pauli!(PauliTuple, pauli_tuple_);

impl_api::pauli_stack!(PauliStack_vb, pauli_stack_vb_, Vec_b);
impl_api::pauli_stack!(PauliStack_bv, pauli_stack_bv_, BitVec);

#![allow(non_camel_case_types)]

use std::{
    hash::BuildHasherDefault,
    mem::ManuallyDrop,
};

use impl_api::*;
use pauli_tracker::{
    collection::{
        BufferedVector,
        Map,
    },
    pauli::{
        PauliDense,
        PauliEnum,
        PauliTuple,
    },
    tracker::{
        live,
        Tracker,
    },
};
use rustc_hash::FxHasher;

/// There are no live_hmpefx_* functions, instead they are named live_*.
type Live_hmpefx = live::Live<Map<PauliEnum, BuildHasherDefault<FxHasher>>>;
type Live = Live_hmpefx;
type Live_bvpd = live::Live<BufferedVector<PauliDense>>;
type Live_bvpt = live::Live<BufferedVector<PauliTuple>>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt),)*) => {$(
        base!($typ, $pre);
        tracker!($typ, $pre);
    )*};
}

boilerplate!((Live, live_), (Live_bvpd, live_bvpd_), (Live_bvpt, live_bvpt_),);

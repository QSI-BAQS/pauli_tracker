#![allow(non_camel_case_types)]

use pauli_tracker::{
    collection::Init,
    tracker::{
        frames::Frames,
        Tracker,
    },
};

use crate::{
    collection::{
        BufferedVector_psbv,
        BufferedVector_psvb,
        Map_psbvfx,
        Map_psvbfx,
        MappedVector_psbvfx,
        MappedVector_psvbfx,
    },
    pauli::{
        PauliStack_bv,
        PauliStack_vb,
    },
};

pub type Frames_hmpsvbfx = Frames<Map_psvbfx>;
pub type Frames_hmpsbvfx = Frames<Map_psbvfx>;
pub type Frames_bvpsvb = Frames<BufferedVector_psvb>;
pub type Frames_bvpsbv = Frames<BufferedVector_psbv>;
pub type Frames_mvpsvbfx = Frames<MappedVector_psvbfx>;
pub type Frames_mvpsbvfx = Frames<MappedVector_psbvfx>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt, $storage:ty, $stack:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::tracker!($typ, $pre, $stack, is_frames);
        impl_api::measure_and_store!($typ, $pre, $storage);
    )*};
}

// actually, one should also include the storage abbreviation in the name, but since we
// always use Map_* as storage, I'm omitting it here (for now)
boilerplate!(
    (Frames_hmpsvbfx, frames_hmpsvbfx_, Map_psvbfx, PauliStack_vb),
    (Frames_hmpsbvfx, frames_hmpsbvfx_, Map_psbvfx, PauliStack_bv),
    (Frames_bvpsvb, frames_bvpsvb_, Map_psvbfx, PauliStack_vb),
    (Frames_bvpsbv, frames_bvpsbv_, Map_psbvfx, PauliStack_bv),
    (Frames_mvpsvbfx, frames_mvpsvb_, MappedVector_psvbfx, PauliStack_vb),
    (Frames_mvpsbvfx, frames_mvpsbv_, MappedVector_psbvfx, PauliStack_bv),
);

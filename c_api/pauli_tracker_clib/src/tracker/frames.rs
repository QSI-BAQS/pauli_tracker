#![allow(non_camel_case_types)]

use pauli_tracker::{
    collection::Init,
    tracker::{
        frames::Frames,
        Tracker,
    },
};

use crate::collection::{
    BufferedVector_psbv,
    BufferedVector_psvb,
    Map_psbvfx,
    Map_psvbfx,
    MappedVector_psbvfx,
    MappedVector_psvbfx,
};

pub type Frames_hmpsvbfx = Frames<Map_psvbfx>;
pub type Frames_hmpsbvfx = Frames<Map_psbvfx>;
pub type Frames_bvpsvb = Frames<BufferedVector_psvb>;
pub type Frames_bvpsbv = Frames<BufferedVector_psbv>;
pub type Frames_mvpsvbfx = Frames<MappedVector_psvbfx>;
pub type Frames_mvpsbvfx = Frames<MappedVector_psbvfx>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt, $storage:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::tracker!($typ, $pre);
        impl_api::measure_and_store!($typ, $pre, $storage);
    )*};
}

boilerplate!(
    (Frames_hmpsvbfx, frames_hmpsvbfx_, Map_psvbfx),
    (Frames_hmpsbvfx, frames_hmpsbvfx_, Map_psbvfx),
    (Frames_bvpsvb, frames_bvpsvb_, Map_psvbfx),
    (Frames_bvpsbv, frames_bvpsbv_, Map_psbvfx),
    (Frames_mvpsvbfx, frames_mvpsvb_, MappedVector_psvbfx),
    (Frames_mvpsbvfx, frames_mvpsbv_, MappedVector_psbvfx),
);

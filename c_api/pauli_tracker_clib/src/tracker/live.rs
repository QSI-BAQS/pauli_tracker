#![allow(non_camel_case_types)]

use pauli_tracker::{
    collection::Init,
    tracker::{
        live,
        Tracker,
    },
};

use crate::collection::{
    BufferedVector_pd,
    BufferedVector_pe,
    BufferedVector_pt,
    Map_pefx,
};

pub type Live_hmpefx = live::Live<Map_pefx>;
pub type Live_bvpe = live::Live<BufferedVector_pe>;
pub type Live_bvpt = live::Live<BufferedVector_pt>;
pub type Live_bvpd = live::Live<BufferedVector_pd>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::tracker!($typ, $pre);
    )*};
}

boilerplate!(
    (Live_hmpefx, live_hmpefx_),
    (Live_bvpe, live_bvpe_),
    (Live_bvpt, live_bvpt_),
    (Live_bvpd, live_bvpd_),
);

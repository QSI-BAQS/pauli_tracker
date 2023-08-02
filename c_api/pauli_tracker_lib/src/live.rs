use std::mem::ManuallyDrop;

use pauli_tracker::{
    collection::BufferedVector,
    pauli::PauliTuple,
    tracker::{
        live,
        Tracker,
    },
};
use impl_api::*;

// type Live = live::Live<BufferedVector<PauliDense>>;
type Live = live::Live<BufferedVector<PauliTuple>>;

#[no_mangle]
pub extern "C-unwind" fn new_live() -> *mut Live {
    ManuallyDrop::new(Box::new(Live::init(0))).as_mut() as *mut Live
}

/// # Safety
#[no_mangle]
pub unsafe extern "C-unwind" fn free_live(live: *mut Live) {
    unsafe { Box::from_raw(live) };
}

tracker_boilerplate!(Live, live_);
serialize!(Live, live_);

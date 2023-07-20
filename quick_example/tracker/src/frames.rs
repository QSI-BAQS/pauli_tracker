use std::mem::ManuallyDrop;

use pauli_tracker::{
    collection::{
        Collection,
        CollectionRequired,
        Map,
    },
    pauli::PauliStack,
    tracker::{
        frames,
        Tracker,
    },
};
use proc::*;

type BoolVec = Vec<bool>;
type Storage = Map<PauliStack<BoolVec>>;
type Frames = frames::Frames<Storage>;

#[no_mangle]
pub extern "C" fn storage_new() -> *mut Storage {
    ManuallyDrop::new(Box::new(Storage::init(0))).as_mut() as *mut Storage
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn storage_free(storage: *mut Storage) {
    unsafe { Box::from_raw(storage) };
}

tracker_boilerplate!(Frames, frames_);

#[no_mangle]
pub extern "C" fn measure_and_store(
    tracker: &mut Frames,
    qubit: usize,
    storage: &mut Storage,
) {
    let _ = tracker.measure_and_store(qubit, storage);
}

// tuples are not supported -> own tuple type
// #[no_mangle]
// pub extern "C" fn sort_by_key(
//     storage: &mut Storage,
// ) -> *mut Vec<(usize, &<Storage as CollectionRequired>::T)> {
//     ManuallyDrop::new(Box::new(storage.sort_by_key())).as_mut() as *mut _
// }

#[no_mangle]
pub extern "C" fn measure_and_store_all(tracker: &mut Frames, storage: &mut Storage) {
    tracker.measure_and_store_all(storage);
}

serialize!(Storage, storage_);

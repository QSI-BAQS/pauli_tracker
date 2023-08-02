use std::{
    hash::BuildHasherDefault,
    mem::ManuallyDrop,
};

use impl_api::*;
use pauli_tracker::{
    collection::Map,
    pauli::PauliStack,
    tracker::{
        frames,
        Tracker,
    },
};
use rustc_hash::FxHasher;

type BoolVec = bit_vec::BitVec;
type Storage = Map<PauliStack<BoolVec>, BuildHasherDefault<FxHasher>>;
type Frames = frames::Frames<Storage>;

base!(Storage, storage_);

base!(Frames, frames_);
tracker!(Frames, frames_);

#[no_mangle]
pub extern "C-unwind" fn measure_and_store(
    tracker: &mut Frames,
    qubit: usize,
    storage: &mut Storage,
) {
    let _ = tracker.measure_and_store(qubit, storage);
}

// tuples are not supported -> own tuple type
// #[no_mangle]
// pub extern "C-unwind" fn sort_by_key(
//     storage: &mut Storage,
// ) -> *mut Vec<(usize, &<Storage as IterableBase>::T)> {
//     ManuallyDrop::new(Box::new(storage.sort_by_key())).as_mut() as *mut _
// }

#[no_mangle]
pub extern "C-unwind" fn measure_and_store_all(
    tracker: &mut Frames,
    storage: &mut Storage,
) {
    tracker.measure_and_store_all(storage);
}

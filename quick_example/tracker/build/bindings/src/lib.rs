#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod frames {
    use std::mem::ManuallyDrop;
    use pauli_tracker::{
        collection::{Collection, CollectionRequired, Map},
        pauli::PauliStack, tracker::{frames, Tracker},
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
    #[no_mangle]
    pub extern "C" fn frames_init(num_qubits: usize) -> *mut Frames {
        ManuallyDrop::new(Box::new(Frames::init(num_qubits))).as_mut() as *mut Frames
    }
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn frames_free(tracker: *mut Frames) {
        unsafe { Box::from_raw(tracker) };
    }
    #[no_mangle]
    pub extern "C" fn frames_track_x(tracker: &mut Frames, qubit: usize) {
        tracker.track_x(qubit);
    }
    #[no_mangle]
    pub extern "C" fn frames_track_y(tracker: &mut Frames, qubit: usize) {
        tracker.track_y(qubit);
    }
    #[no_mangle]
    pub extern "C" fn frames_track_z(tracker: &mut Frames, qubit: usize) {
        tracker.track_z(qubit);
    }
    #[no_mangle]
    pub extern "C" fn frames_h(tracker: &mut Frames, qubit: usize) {
        tracker.h(qubit);
    }
    #[no_mangle]
    pub extern "C" fn frames_s(tracker: &mut Frames, qubit: usize) {
        tracker.s(qubit);
    }
    #[no_mangle]
    pub extern "C" fn frames_cx(tracker: &mut Frames, control: usize, target: usize) {
        tracker.cx(control, target);
    }
    #[no_mangle]
    pub extern "C" fn frames_cz(tracker: &mut Frames, qubit_a: usize, qubit_b: usize) {
        tracker.cz(qubit_a, qubit_b);
    }
    #[no_mangle]
    pub extern "C" fn frames_new_qubit(tracker: &mut Frames, qubit: usize) {
        tracker.new_qubit(qubit);
    }
    #[no_mangle]
    pub extern "C" fn measure_and_store(
        tracker: &mut Frames,
        qubit: usize,
        storage: &mut Storage,
    ) {
        let _ = tracker.measure_and_store(qubit, storage);
    }
    #[no_mangle]
    pub extern "C" fn measure_and_store_all(
        tracker: &mut Frames,
        storage: &mut Storage,
    ) {
        tracker.measure_and_store_all(storage);
    }
    #[no_mangle]
    pub extern "C" fn storage_serialize(
        storage: &Storage,
        file: *const std::ffi::c_char,
    ) {
        let file = unsafe { std::ffi::CStr::from_ptr(file as *const i8) }
            .to_str()
            .unwrap();
        let output = serde_json::to_string(storage).unwrap();
        std::fs::write(file, output).unwrap();
    }
}
pub mod live {
    use std::mem::ManuallyDrop;
    use pauli_tracker::{
        collection::BufferedVector, pauli::PauliTuple, tracker::{live, Tracker},
    };
    use proc::*;
    type Live = live::Live<BufferedVector<PauliTuple>>;
    #[no_mangle]
    pub extern "C" fn new_live() -> *mut Live {
        ManuallyDrop::new(Box::new(Live::init(0))).as_mut() as *mut Live
    }
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn free_live(live: *mut Live) {
        unsafe { Box::from_raw(live) };
    }
    #[no_mangle]
    pub extern "C" fn live_init(num_qubits: usize) -> *mut Live {
        ManuallyDrop::new(Box::new(Live::init(num_qubits))).as_mut() as *mut Live
    }
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn live_free(tracker: *mut Live) {
        unsafe { Box::from_raw(tracker) };
    }
    #[no_mangle]
    pub extern "C" fn live_track_x(tracker: &mut Live, qubit: usize) {
        tracker.track_x(qubit);
    }
    #[no_mangle]
    pub extern "C" fn live_track_y(tracker: &mut Live, qubit: usize) {
        tracker.track_y(qubit);
    }
    #[no_mangle]
    pub extern "C" fn live_track_z(tracker: &mut Live, qubit: usize) {
        tracker.track_z(qubit);
    }
    #[no_mangle]
    pub extern "C" fn live_h(tracker: &mut Live, qubit: usize) {
        tracker.h(qubit);
    }
    #[no_mangle]
    pub extern "C" fn live_s(tracker: &mut Live, qubit: usize) {
        tracker.s(qubit);
    }
    #[no_mangle]
    pub extern "C" fn live_cx(tracker: &mut Live, control: usize, target: usize) {
        tracker.cx(control, target);
    }
    #[no_mangle]
    pub extern "C" fn live_cz(tracker: &mut Live, qubit_a: usize, qubit_b: usize) {
        tracker.cz(qubit_a, qubit_b);
    }
    #[no_mangle]
    pub extern "C" fn live_new_qubit(tracker: &mut Live, qubit: usize) {
        tracker.new_qubit(qubit);
    }
    #[no_mangle]
    pub extern "C" fn live_serialize(storage: &Live, file: *const std::ffi::c_char) {
        let file = unsafe { std::ffi::CStr::from_ptr(file as *const i8) }
            .to_str()
            .unwrap();
        let output = serde_json::to_string(storage).unwrap();
        std::fs::write(file, output).unwrap();
    }
}

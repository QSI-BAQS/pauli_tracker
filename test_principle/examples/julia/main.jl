#!/usr/bin/env julia

include("lib.jl")

tracker = new_tracker()
storage = new_storage()

new_qubit(tracker, UInt(0))
new_qubit(tracker, UInt(1))
new_qubit(tracker, UInt(2))

# qubit1 qubit2 qubit3
# xz     xz     xz

track_x(tracker, UInt(1))
# 00 10 00
track_z(tracker, UInt(2))
# 00 10 00
# 00 00 01
apply_cx(tracker, UInt(1), UInt(2))
# 00 10 10
# 00 01 01
apply_cx(tracker, UInt(2), UInt(0))
# 10 10 10
# 00 01 01

# =
# 10 12 12

for i in range(0, 2)
    measure_and_store(tracker, UInt(i), storage)
end

raw_sorted = sort_storage(storage)

sorted = unsafe_wrap(Vector{Tuple}, raw_sorted.ptr, raw_sorted.len)

function show(pauli)
    println(unsafe_wrap(Vector{UInt32}, pauli.ptr, pauli.len))
end

for t in sorted
    println(t.qubit)
    pauli = raw_pauli_vec(t.pauli).left
    show(pauli)
    pauli = raw_pauli_vec(t.pauli).right
    show(pauli)
    println()
end

free_sorted_storage(raw_sorted)
free_storage(storage)
free_tracker(tracker)

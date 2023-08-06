output := output

.PHONY: build libs header

build: libs header

libs: | $(output)
	@(cd pauli_tracker_clib && cargo build --release)
	@cp pauli_tracker_clib/target/release/libpauli_tracker_clib.{a,so} $(output)/

header: | $(output)
	@(cd generate_bindings && cargo run)
	@cp generate_bindings/output/pauli_tracker.h $(output)/

$(output):
	@mkdir -p output

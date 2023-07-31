build: | output
	@(cd pauli_tracker_lib && cargo build --release)
	@(cd generate_bindings && cargo run)
	@cp pauli_tracker_lib/target/release/libpauli_tracker.{a,so} output/
	@cp generate_bindings/output/* output/

output:
	@mkdir -p output

output := output
lib = libpauli_tracker_clib
lib_path = pauli_tracker_clib/target/release/$(lib)
libs = $(lib_path).a $(lib_path).so
header = pauli_tracker.h
header_path = generate_bindings/output/$(header)


build: $(output)/ $(output)/$(lib).so $(output)/$(header)


$(output)/$(lib).so: $(lib_path).so
	cp $(libs) $(output)/

$(lib_path).so: FORCE
	(cd pauli_tracker_clib && cargo build --release)
	# (cd pauli_tracker_clib && CARGO_PROFILE_RELEASE_LTO=true \
	# 	RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-fuse-ld=lld" \
	# 	cargo +stable build --release)


$(output)/$(header): $(header_path)
	cp $(header_path) $(output)/

$(header_path): FORCE
	(cd generate_bindings && cargo run --release)


%/:
	@mkdir -p $@

clean:
	rm -rf $(output)
	(cd pauli_tracker_clib && cargo clean)
	(cd generate_bindings && cargo clean && rm -rf output)


FORCE:

.PHONY: FORCE build

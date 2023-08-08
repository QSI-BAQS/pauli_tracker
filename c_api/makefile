output := output
lib = libpauli_tracker_clib
lib_path = pauli_tracker_clib/target/release/$(lib)
header = pauli_tracker.h
header_path = generate_bindings/output/$(header)


build: $(output)/ $(output)/$(lib).so $(output)/$(header)


$(output)/$(lib).so: $(lib_path).so
	cp $(lib_path).{a,so} $(output)/

$(lib_path).so: FORCE
	(cd pauli_tracker_clib && cargo build --release)


$(output)/$(header): $(header_path)
	cp $(header_path) $(output)/

$(header_path): FORCE
	(cd generate_bindings && cargo run)


%/:
	@mkdir -p $@

clean:
	rm -rf $(output)


FORCE: ;

.PHONY: build

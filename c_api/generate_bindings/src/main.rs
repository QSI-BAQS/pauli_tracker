use gen_bind::{
    Generator,
    GeneratorConfig,
};

fn main() {
    Generator::with_config(
        "pauli_tracker_lib",
        GeneratorConfig::new()
            .crate_dir("../pauli_tracker_lib")
            .output_dir("output")
            .header_name("pauli_tracker"),
    )
    .setup()
    .generate();
}

#[test]
fn compile_bin() {
    let mut instance = nasm_build::Instance::new("tests/test.s");
    instance.set_format(nasm_build::OutputFormat::Binary);
    let output_filepath = instance.compile().unwrap();

    println!("{}", &output_filepath.display());

    std::fs::remove_file(output_filepath).expect("Failed to clean up after test");
}

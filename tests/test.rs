#[test]
fn compile_bin() {
    let mut instance = nasm::Instance::new("tests/test.s");
    instance.set_format(nasm::OutputFormat::Binary);
    let output_filepath = instance.compile().unwrap();

    println!("{}", &output_filepath.display());

    std::fs::remove_file(output_filepath).expect("Failed to clean up after test");
}

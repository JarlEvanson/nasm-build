#[test]
fn compile_bin() {
    let mut instance = nasm_build::Instance::new("tests/test.s");
    instance.set_format(nasm_build::OutputFormat::Binary);
    let output_filepath = instance.compile().unwrap();

    println!("{}", &output_filepath.display());

    std::fs::remove_file(output_filepath).expect("Failed to clean up after test");
}

#[test]
fn directories_completion() {
    let mut instance = nasm_build::Instance::new("tests/test.s");
    instance.set_format(nasm_build::OutputFormat::Binary);
    instance.set_output("build/tests.rs");
    let output_filepath = instance.compile().unwrap();

    println!("{}", &output_filepath.display());

    assert!(output_filepath.parent().unwrap().exists());

    std::fs::remove_file(&output_filepath).expect("Failed to clean up after test");
    std::fs::remove_dir(&output_filepath.parent().unwrap()).expect("Failed to clean up after test");
}

#[test]
fn compile_nasm() {
    let mut instance = nasm::Instance::new();
    instance.set_format(nasm::OutputFormat::Binary);
    instance.compile().unwrap();
}

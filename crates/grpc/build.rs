fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=proto/spbased.proto");
    tonic_build::compile_protos("proto/spbased.proto")?;
    Ok(())
}

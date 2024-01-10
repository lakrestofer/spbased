fn main() -> Result<(), Box<dyn std::error::Error>> {
    // this injects cargo version, rustc version etc such that it is reachable for env!
    vergen::EmitBuilder::builder()
        .all_build()
        .all_git()
        .emit()?;
    Ok(())
}

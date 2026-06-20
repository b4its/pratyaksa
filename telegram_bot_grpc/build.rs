fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile definisi gRPC dari proto/alert.proto saat build
    tonic_build::compile_protos("proto/alert.proto")?;
    Ok(())
}

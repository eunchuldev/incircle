use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/request.proto", "src/response.proto"], 
        &["src/"])?;
    Ok(())
}

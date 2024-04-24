use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    tauri_build::build();
    // Emit the instructions
    EmitBuilder::builder().all_git().emit()?;
    Ok(())
}

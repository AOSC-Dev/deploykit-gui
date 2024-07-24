use std::error::Error;

use vergen::Emitter;
use vergen_gix::GixBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    tauri_build::build();
    // Emit the instructions
    let gix = GixBuilder::all_git()?;
    Emitter::default().add_instructions(&gix)?.emit()?;

    Ok(())
}

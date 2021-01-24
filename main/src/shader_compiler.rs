use anyhow::Result;

pub fn compile_shader<F>(device: &metal::Device, mut on_compiled: F) -> Result<metal::Library>
where
  F: FnMut(String),
{
  let root_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let shader_prelude = std::fs::read_to_string(root_path.join("src/shader_prelude.metal"))?;
  let mut shader = shader_prelude;
  {
    let fragment_shader_in_rust = std::fs::read_to_string(root_path.join("src/shader.rs"))?;
    let fragment_shader_in_msl =
      crate::transpiler::transpile_rust_to_msl(&fragment_shader_in_rust)?;
    shader.push_str(&fragment_shader_in_msl);
    on_compiled(fragment_shader_in_msl);
  }
  let library = device
    .new_library_with_source(&shader, &metal::CompileOptions::new())
    .map_err(anyhow::Error::msg)?;
  Ok(library)
}

#[test]
fn test() -> Result<()> {
  compile_shader(&metal::Device::system_default().unwrap(), |shader_in_msl| {
    println!("{}", shader_in_msl)
  })?;
  Ok(())
}

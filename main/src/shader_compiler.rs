use anyhow::Result;

lazy_static::lazy_static! {
  static ref ROOT_PATH: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  pub static ref SHADER_PATH: std::path::PathBuf = ROOT_PATH.join("src/shader.rs");
  pub static ref SHADER_PRELUDE_PATH: std::path::PathBuf = ROOT_PATH.join("src/shader_prelude.metal");
}

pub fn compile_shader<F>(device: &metal::Device, mut on_compiled: F) -> Result<metal::Library>
where
  F: FnMut(String),
{
  let shader_prelude = std::fs::read_to_string(&*SHADER_PRELUDE_PATH)?;
  let mut shader = shader_prelude;
  {
    let fragment_shader_in_rust = std::fs::read_to_string(&*SHADER_PATH)?;
    let fragment_shader_in_msl = rust_to_metal_sl::transpile(&fragment_shader_in_rust)?;
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

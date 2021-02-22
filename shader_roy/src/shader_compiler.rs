use anyhow::{Context, Result};

const ENTRY_POINT_FN_NAME: &str = "pixel_color";

lazy_static::lazy_static! {
  static ref ROOT_PATH: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  pub static ref SHADER_PRELUDE_PATH: std::path::PathBuf =
    ROOT_PATH.join("src/shader_prelude.metal");
  pub static ref SHADER_INTERFACE_PATH: std::path::PathBuf =
    ROOT_PATH.join("../shader_roy_metal_sl_interface/src/shader_roy_metal_sl_interface.rs");
}

pub fn compile_shader<F>(
  shader_file_path: &std::path::Path,
  device: &metal::Device,
  mut on_compiled: F,
) -> Result<metal::Library>
where
  F: FnMut(String),
{
  let shader_prelude = std::fs::read_to_string(&*SHADER_PRELUDE_PATH)?;
  let shader_interface = std::fs::read_to_string(&*SHADER_INTERFACE_PATH)?;
  let combined_shader = {
    let fragment_shader_in_rust = std::fs::read_to_string(shader_file_path)
      .with_context(|| format!("Failed to read shader from `{:?}`", shader_file_path))?;
    let config = rust_to_metal_sl::EnhanceConfig {
      entry_point_fn_name: ENTRY_POINT_FN_NAME.to_owned(),
      constant_name: "INPUT".to_owned(),
      param_type: "Input".to_owned(),
    };
    let fragment_interface_in_msl = rust_to_metal_sl::transpile(&shader_interface, &config)?;
    let fragment_shader_in_msl = rust_to_metal_sl::transpile(&fragment_shader_in_rust, &config)?;
    let combined_shader =
      // voca_rs::manipulate::replace(&shader_prelude, "/// SHADER_RS", &fragment_shader_in_msl);
      shader_prelude.replace(
          "SHADER_RS_ENTRYPOINT",
          "pixel_color",
        ).replace(
          "/// SHADER_RS",
          &format!("{}{}", fragment_interface_in_msl, fragment_shader_in_msl),
        );
    on_compiled(fragment_shader_in_msl);
    combined_shader
  };
  let library = device
    .new_library_with_source(&combined_shader, &metal::CompileOptions::new())
    .map_err(anyhow::Error::msg)?;
  Ok(library)
}

#[test]
fn test() -> Result<()> {
  compile_shader(
    ROOT_PATH
      .join("../example_raymarching/src/example_raymarching.rs")
      .as_path(),
    &metal::Device::system_default().unwrap(),
    |shader_in_msl| println!("{}", shader_in_msl),
  )?;
  Ok(())
}

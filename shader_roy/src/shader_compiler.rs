use anyhow::{Context, Result};
use path_absolutize::Absolutize;

const ENTRY_POINT_FN_NAME: &str = "pixel_color";

lazy_static::lazy_static! {
  pub static ref ROOT_PATH: std::path::PathBuf =
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
    let fragment_shader_in_rust = read_shader_sources(shader_file_path)?;
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

fn read_shader_sources(shader_file_path: &std::path::Path) -> Result<String> {
  let fragment_shader_in_rust = std::fs::read_to_string(shader_file_path).with_context(|| {
    format!(
      "Failed to read shader from `{:?}`",
      shader_file_path.absolutize().unwrap()
    )
  })?;
  lazy_static::lazy_static! {
      static ref MODULE_DECLARATION: regex::Regex =
        regex::Regex::new(r"mod\s+(?P<module_name>\w+)\s*;").unwrap();
  }
  let shader_directory = shader_file_path.parent().unwrap().to_owned();

  use regex_try::RegexTry;
  MODULE_DECLARATION
    .try_replace_all(&fragment_shader_in_rust, |captured: &regex::Captures| {
      read_shader_sources(
        &shader_directory
          .join(&captured["module_name"])
          .with_extension("rs"),
      )
    })
    .map(|result| result.into_owned())
}

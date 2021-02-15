use anyhow::{anyhow, Context, Result};
use path_absolutize::Absolutize;

pub fn get_path() -> Result<std::path::PathBuf> {
  let args: Vec<String> = std::env::args().collect();
  let arg_path = &std::path::PathBuf::from(
    args
      .get(1)
      .ok_or_else(|| anyhow!("Missing shader entry point file path argument"))?,
  );
  path_or_lib_src(arg_path).map_err(|error| {
    anyhow!(
      "{:?}\
      \n\nFor argument {:?} (absolute path {:?})",
      error,
      arg_path,
      arg_path.absolutize().unwrap()
    )
  })
}

fn path_or_lib_src(arg_path: &std::path::PathBuf) -> Result<std::path::PathBuf> {
  let path_info = std::fs::metadata(arg_path)
    .with_context(|| "File path argument doesn\'t match an existing directory or file.")?;
  Ok(if path_info.is_dir() {
    let cargo_manifest_path = &arg_path.join("Cargo.toml");
    let cargo_manifest = std::fs::read_to_string(cargo_manifest_path).with_context(|| {
      "File path matches a directory, but the directory is missing the expected Cargo.toml file."
    })?;
    let parsed_cargo_manifest = cargo_manifest.parse::<toml::Value>().with_context(|| {
      format!(
        "Could not parse Cargo.toml file at {:?}",
        cargo_manifest_path
      )
    })?;
    if let Some(path) = parsed_cargo_manifest["lib"]["path"].as_str() {
      arg_path.join(path)
    } else {
      arg_path.join("src/lib.rs")
    }
  } else {
    arg_path.to_owned()
  })
}
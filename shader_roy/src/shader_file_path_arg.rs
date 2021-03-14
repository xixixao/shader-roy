use anyhow::{anyhow, Context, Result};
use path_absolutize::Absolutize;

lazy_static::lazy_static! {
  pub static ref ROOT_PATH: std::path::PathBuf =
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
}

pub fn get_path() -> Result<std::path::PathBuf> {
  let args: Vec<String> = std::env::args().collect();
  get_path_for_argument(args.get(1).ok_or_else(|| {
    anyhow!("Cannot start ShaderRoy: Missing shader entry point file path argument")
  })?)
}

pub fn get_path_for_argument(argument: &str) -> Result<std::path::PathBuf> {
  let arg_path = &std::path::PathBuf::from(argument);
  path_or_lib_src(arg_path).map_err(|error| {
    anyhow!(
      "Cannot start ShaderRoy: {:?}\
      \n\nFor argument {:?} (absolute path {:?})",
      error,
      arg_path,
      arg_path.absolutize().unwrap()
    )
  })
}

fn path_or_lib_src(arg_path: &std::path::PathBuf) -> Result<std::path::PathBuf> {
  let (resolved_path, path_info) = std::fs::metadata(arg_path)
    .map(|info| (arg_path.to_owned(), info))
    .or_else(|_| {
      let example_path = ROOT_PATH.join("../examples").join(arg_path);
      std::fs::metadata(&example_path).map(|info| (example_path, info))
    })
    .with_context(|| "File path argument doesn\'t match an existing directory or file.")?;
  Ok(if path_info.is_dir() {
    let cargo_manifest_path = &resolved_path.join("Cargo.toml");
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
      let source_path = resolved_path.join(path);
      std::fs::metadata(&source_path).with_context(|| {
        format!(
          "Could not find the source file at {:?} based on [lib][path] {} specified in Cargo.toml file at {:?}",
          source_path,
          path,
          cargo_manifest_path
        )
      })?;
      source_path
    } else {
      resolved_path.join("src/lib.rs")
    }
  } else {
    resolved_path
  })
}

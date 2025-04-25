use std::path::PathBuf;

use glob::glob;

pub(crate) fn workspace_config_filenames() -> Result<(Vec<String>, String), String> {
    let workspace_dir = match env!("CARGO_WORKSPACE_DIR").trim().trim_end_matches('/') {
        "" => Err("\
            CARGO_WORKSPACE_DIR environment variable not defined; this should be in the workspace's .cargo/config.toml and \
            you need to be running the build via cargo"),
        non_empty => Ok(non_empty)
    }?;

    let default_config_filenames = all_hierarchical_config_filenames_in(workspace_dir)?;
    let override_config_filename = format!("{workspace_dir}/build_config.toml");
    Ok((default_config_filenames, override_config_filename))
}

fn all_hierarchical_config_filenames_in(workspace_dir: &str) -> Result<Vec<String>, String> {
    let crates_dir = format!("{}/crates", workspace_dir);
    let hierarchy_dirs = [
        ("kernel", false),
        ("mcus", true),
        ("mcus", false),
        ("drivers", true),
        ("drivers", false),
        ("boards", true),
        ("boards", false)];

    let filenames = unwrap(&mut hierarchy_dirs
        .iter()
        .map(|(subdir, is_family)|
            match all_filenames_matching(
                &format!(
                    "{crates_dir}/{subdir}/**{}/build_config.toml",
                    if *is_family { "/*_family/**" } else { "" })) {

                Ok(filenames) if *is_family => Ok(filenames),

                Ok(filenames) => Ok(filenames
                    .iter()
                    .filter(|filename| !filename.contains("_family/"))
                    .map(|f| f.to_owned())
                    .collect::<Vec<_>>()),

                Err(oops) => Err(oops)
        }))?;

    Ok(filenames)
}

fn unwrap<E, I: Iterator<Item = Result<Vec<String>, E>>>(fallible: &mut I) -> Result<Vec<String>, E> {
    Ok(fallible
        .collect::<Result<Vec<Vec<_>>, _>>()?
        .iter()
        .flatten()
        .cloned()
        .collect())
}

fn all_filenames_matching(pattern: &str) -> Result<Vec<String>, String> {
    all_pathbufs_matching(pattern)?.iter().map(|f| match f.to_str() {
        Some(utf8) => Ok(utf8.to_string()),
        None => Err(format!("Pattern glob resulted in non-UTF8 filenames (probably); pattern={pattern}"))
    }).collect::<Result<Vec<String>, String>>()
}

fn all_pathbufs_matching(pattern: &str) -> Result<Vec<PathBuf>, String> {
    match glob(pattern) {
        Ok(filenames) => filenames.collect::<Result<_, _>>().map_err(|err| err.to_string()),
        Err(error) => Err(error.to_string())
    }
}

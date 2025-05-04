use std::{error::Error, fs::canonicalize, path::PathBuf};

use glob::glob;

use smeg_build_utils::results::*;

pub(crate) fn workspace_config_filenames_in(workspace_dir: &str) -> Result<(Vec<String>, String), StringError> {
    let workspace_dir = workspace_dir.trim().trim_end_matches('/');
    if workspace_dir.is_empty() {
        return Err(StringError::from("\
            No workspace_dir argument passed and no CARGO_WORKSPACE_DIR environment variable defined; the latter should be \
            pre-defined in the workspace's .cargo/config.toml and you need to be running the build via cargo"));
    }

    let default_config_filenames = all_hierarchical_config_filenames_in(workspace_dir)?;
    let override_config_filename = override_config_filename_in(workspace_dir)?;

    Ok((default_config_filenames, override_config_filename))
}

fn all_hierarchical_config_filenames_in(workspace_dir: &str) -> ResultAnyError<Vec<String>> {
    let crates_dir = format!("{}/crates", workspace_dir);
    let hierarchy_dirs = [
        ("kernel", false),
        ("mcus", true),
        ("mcus", false),
        ("drivers", true),
        ("drivers", false),
        ("boards", true),
        ("boards", false)];

    let mut filenames = Vec::<String>::new();
    hierarchy_dirs.iter().try_for_each(|(subdir, is_family)| {
        match all_filenames_matching(
            &format!(
                "{crates_dir}/{subdir}/**{}/smeg_config.toml",
                if *is_family { "/*_family/**" } else { "" })) {

            Ok(mut unfiltered_filenames) => {
                unfiltered_filenames.drain(..).try_for_each(|filename| -> ResultAnyError<()> {
                    let unprefixed_filename = &filename[workspace_dir.len() ..];
                    if !unprefixed_filename.contains("/tests/") && (*is_family || !unprefixed_filename.contains("_family/")) {
                        filenames.push(absolute_path_to(&filename)?);
                    }
                    Ok(())
                })
            },

            Err(err) => Err(err)
        }
    })?;

    Ok(filenames)
}

fn all_filenames_matching(pattern: &str) -> ResultAnyError<Vec<String>> {
    all_pathbufs_matching(pattern)?.iter().map(|f| match f.to_str() {
        Some(utf8) => Ok(utf8.to_string()),
        None => Err(format!("Pattern glob resulted in non-UTF8 filenames (probably); pattern={pattern}").into())
    }).collect::<ResultAnyError<Vec<String>>>()
}

fn all_pathbufs_matching(pattern: &str) -> ResultAnyError<Vec<PathBuf>> {
    glob(pattern)?.map(|result| match result {
        Ok(filename) => Ok(filename),
        Err(err) => Err(Box::<dyn Error>::from(err))
    }).collect()
}

fn absolute_path_to(filename: &str) -> ResultAnyError<String> {
    Ok(canonicalize(filename)?
        .to_str()
        .ok_or_else(|| format!("Filename cannot be canonicalised (non-UTF8 ?); filename={filename}"))?
        .to_string())
}

fn override_config_filename_in(workspace_dir: &str) -> ResultAnyError<String> {
    let override_config_filename = format!("{workspace_dir}/smeg_config.toml");
    let override_config_filename = all_filenames_matching(&override_config_filename)?
        .pop()
        .ok_or_else(|| format!("There is no override config; expected_filename={override_config_filename}"))?;

    absolute_path_to(&override_config_filename)
}

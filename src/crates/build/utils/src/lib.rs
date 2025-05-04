use std::{fs::canonicalize, path::PathBuf};

pub mod results;
use results::{ResultAnyError, StringError};

pub fn smeg_out_dir() -> Result<String, StringError> {
    try_get_smeg_out_dir().map_err(|err|
        StringError::from(format!("\
            Could not determine the smeg output (target) directory.  Possible causes include not setting the SMEG_OUT_DIR environment variable (ie. \
            running outside of build.sh) or non-UTF8 characters in the canonical representation; err={err}")))
}

fn try_get_smeg_out_dir() -> ResultAnyError<String> {
    let smeg_out_dir = canonicalize(PathBuf::from(&std::env::var("SMEG_OUT_DIR")?))?;
    Ok(smeg_out_dir
        .to_str()
        .ok_or_else(|| StringError::from("Canonical representation was not possible; non-UTF8 characters in the path ?"))?
        .trim_end_matches('/')
        .to_string())
}

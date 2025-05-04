use smeg_build_utils::results::StringError;

pub struct WorkspaceConfig {
    dir: String
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            dir: sanitised_workspace_dir(CARGO_WORKSPACE_DIR)
        }
    }
}

const CARGO_WORKSPACE_DIR: &str = env!("CARGO_WORKSPACE_DIR");

fn sanitised_workspace_dir(dir: &str) -> String {
    let workspace_dir = sanitised_dir(CARGO_WORKSPACE_DIR);
    sanitised_dir(&dir.replace("${workspace_dir}", &workspace_dir))
}

fn sanitised_dir(dir: &str) -> String {
    dir.trim_start().trim_end_matches(|ch: char| ch == '/' || ch.is_whitespace()).to_string()
}

impl WorkspaceConfig {
    pub fn dir(&self) -> Result<&str, StringError> {
        if self.dir.is_empty() {
            Err(StringError::from("\
                No workspace_dir argument passed and no CARGO_WORKSPACE_DIR environment variable defined; the latter should be \
                pre-defined in the workspace's .cargo/config.toml and you need to be running the build via cargo"))

        } else {
            Ok(&self.dir)
        }
    }

    pub fn parse_mut(&mut self, args: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if args.path.is_ident("workspace_dir") {
            self.dir = sanitised_workspace_dir(&args.value()?.parse::<syn::LitStr>()?.value());
            Ok(())
        } else {
            Err(args.error("unknown argument for workspace config"))
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;
    use syn::parse_quote;

    use smeg_testing_host_utils::strings::utf8;
    use super::*;

    #[test]
    fn sanitised_workspace_dir__called_with_trailing_slash__expect_trailing_slash_is_removed() {
        let with_trailing_slash = format!("{}/", any_dir());
        expect!(sanitised_workspace_dir(&with_trailing_slash)).not().to_end_with("/");
    }

    fn any_dir() -> String {
        utf8::any_nonempty()
    }

    #[test]
    fn sanitised_workspace_dir__called_with_trailing_whitespace_and_slash__expect_trailing_whitespace_is_removed() {
        let with_trailing_whitespace_and_slash = format!("{}     /", any_dir());
        expect!(sanitised_workspace_dir(&with_trailing_whitespace_and_slash)).not().to_end_with(" ");
    }

    #[test]
    fn sanitised_workspace_dir__called_with_trailing_slash_and_whitespace__expect_trailing_slash_is_removed() {
        let with_trailing_slash_and_whitespace = format!("{}/     ", any_dir());
        expect!(sanitised_workspace_dir(&with_trailing_slash_and_whitespace)).not().to_end_with("/");
    }

    #[test]
    fn sanitised_workspace_dir__called_with_whitespace__expect_empty_string_returned() {
        let whitespace = utf8::any_whitespace();
        expect!(sanitised_workspace_dir(&whitespace)).to_be_empty();
    }

    #[test]
    fn sanitised_workspace_dir__called_with_workspace_dir_placeholder__expect_environment_variable_is_substituted() {
        let workspace_dir_env = CARGO_WORKSPACE_DIR.trim_start().trim_end_matches(|ch: char| ch == '/' || ch.is_whitespace());
        let substituted = sanitised_workspace_dir(&format!(
            "{}${{workspace_dir}}{}/{}",
            utf8::any_whitespace(),
            utf8::any_whitespace(),
            utf8::any_whitespace()));

        expect!(substituted.as_str()).to_equal(workspace_dir_env);
    }

    #[test]
    fn workspace_config_dir__called_after_default_construction__expect_sanitised_cargo_workspace_environment_variable() {
        let sanitised_workspace_dir_env = sanitised_dir(CARGO_WORKSPACE_DIR);
        let config = WorkspaceConfig::default();
        let dir = config.dir();
        expect!(dir.is_ok()).to_be_true();
        expect!(dir.unwrap()).to_equal(&sanitised_workspace_dir_env);
    }

    #[test]
    fn workspace_config_dir__called_when_empty__expect_err() {
        let config = WorkspaceConfig { dir: "".to_string() };
        let dir = config.dir();
        expect!(dir.is_err()).to_be_true();
        expect!(dir.err().unwrap().to_string()).to_start_with("No workspace_dir argument");
    }

    #[test]
    fn workspace_config_dir__get_after_parse_mut__expect_sanitised_directory() {
        let prefix = utf8::any().trim().trim_matches('/').to_string();
        let suffix = utf8::any().trim().trim_matches('/').to_string();
        let whitespace = utf8::any_whitespace();
        let unsanitised_workspace_dir = format!("{whitespace}/{prefix}a/b${{workspace_dir}}/c{suffix}{whitespace}/{whitespace}");
        let sanitised_workspace_dir = format!("/{prefix}a/b{}/c{suffix}", sanitised_dir(CARGO_WORKSPACE_DIR));
        let attr: syn::Attribute = parse_quote! { #[whatever(workspace_dir = #unsanitised_workspace_dir)] };
        let mut config = WorkspaceConfig::default();
        expect!(attr.parse_nested_meta(|meta| config.parse_mut(meta))).to_be_ok();
        expect!(config.dir).to_equal(sanitised_workspace_dir);
    }

    #[test]
    fn workspace_config_parse_mut__called_with_unknown_argument__expect_err() {
        let unknown_arguments: Vec<syn::Attribute> = vec![
            parse_quote! { #[whatever(workspace_dir_ = "/tmp/whatever")] },
            parse_quote! { #[whatever(workspace__dir = "/tmp/whatever")] },
            parse_quote! { #[whatever(_workspace_dir = "/tmp/whatever")] },
            parse_quote! { #[whatever(workspacedir = "/tmp/whatever")] },
            parse_quote! { #[whatever(Workspace_dir = "/tmp/whatever")] },
            parse_quote! { #[whatever(workspace_Dir = "/tmp/whatever")] }
        ];

        for unknown_argument in unknown_arguments {
            let mut config = WorkspaceConfig::default();
            let result = unknown_argument.parse_nested_meta(|meta| config.parse_mut(meta));
            expect!(&result).to_be_err();
            expect!(result.err().unwrap().to_string()).to_contain("unknown argument");
        }
    }

    #[test]
    fn workspace_config_parse_mut__called_with_valueless_argument__expect_err() {
        let mut config = WorkspaceConfig::default();
        let malformed_argument: syn::Attribute = parse_quote! { #[whatever(workspace_dir)] };
        let result = malformed_argument.parse_nested_meta(|meta| config.parse_mut(meta));
        expect!(&result).to_be_err();
        expect!(result.err().unwrap().to_string()).to_contain("expected").and().to_contain("=");
    }

    #[test]
    fn workspace_config_parse_mut__called_with_non_string_argument__expect_err() {
        let malformed_arguments: Vec<syn::Attribute> = vec![
            parse_quote! { #[whatever(workspace_dir = 123)] },
            parse_quote! { #[whatever(workspace_dir = 45.6)] },
            parse_quote! { #[whatever(workspace_dir = 'x')] },
            parse_quote! { #[whatever(workspace_dir = { "/tmp/whatever" })] },
            parse_quote! { #[whatever(workspace_dir = [ "/tmp/whatever" ])] },
            parse_quote! { #[whatever(workspace_dir = ())] }
        ];

        for malformed_argument in malformed_arguments {
            let mut config = WorkspaceConfig::default();
            let result = malformed_argument.parse_nested_meta(|meta| config.parse_mut(meta));
            expect!(&result).to_be_err();
            expect!(result.err().unwrap().to_string()).to_contain("expected").and().to_contain("string");
        }
    }
}

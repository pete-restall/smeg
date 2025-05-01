use std::{fs::read_to_string, sync::OnceLock};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use regex::{Captures, Regex};
use serde_toml_merge::merge;
use syn::{token::Pub, DeriveInput, Ident, Visibility};
use toml::{Table, Value};

// TODO: All of this hackery needs tidying up and testing
pub(crate) fn load_from(default_config_filenames: &[String], override_config_filename: &String, input: &DeriveInput) -> Result<TokenStream, String> {
    let (struct_attributes, struct_visibility, struct_name) = (&input.attrs, &input.vis, &input.ident);

    let merged_default_configs = default_config_filenames.iter().try_fold(Value::Table(Table::default()), try_loading_and_merging_from_file)?;
    let merged_configs = try_loading_and_merging_from_file(merged_default_configs, override_config_filename)?;
    let parsed_config = ParsedConfig::new(&merged_configs);

    let config_struct_declarations = parsed_config.tokens_for_struct_declarations(&Visibility::Public(Pub(Span::call_site())))?;
    let config_field_values = parsed_config.tokens_for_field_values()?;

    let generated_mod_name = format_ident!("{struct_name}_generated");

    Ok(quote! {
        #[derive(Debug, PartialEq)]
        #( #struct_attributes )*
        #struct_visibility struct #struct_name {
            pub VALUES: #generated_mod_name::Config
        }

        impl #struct_name {
            pub const fn new() -> Self {
                Self {
                    VALUES: #generated_mod_name::Config::new()
                }
            }

            pub const fn default() -> Self {
                Self {
                    VALUES: #generated_mod_name::Config::default()
                }
            }
        }

        #struct_visibility mod #generated_mod_name {
            #config_struct_declarations

            impl Config {
                pub const fn new() -> Self { Self::default() }

                pub const fn default() -> Self {
                    #config_field_values
                }
            }
        }
    })
}

fn try_loading_and_merging_from_file(existing: Value, config_filename: &String) -> Result<Value, String> {
    match read_to_string(config_filename) {
        Ok(toml) => try_merging_with_toml(existing, &toml),
        Err(oops) => Err(oops.to_string())
    }
}

fn try_merging_with_toml(existing: Value, toml: &str) -> Result<Value, String> {
    match toml.parse() {
        Ok(parsed) => try_merging_values(existing, parsed),
        Err(oops) => Err(oops.to_string())
    }
}

fn try_merging_values(existing: Value, additional: Value) -> Result<Value, String> {
    match merge(existing, additional) {
        Ok(merged) => Ok(merged),
        Err(oops) => Err(oops.to_string())
    }
}

#[derive(Debug)]
struct NamedTable<'a> {
    path: String,
    as_value: &'a Value,
    as_map: &'a Table
}

// TODO: paths should really be an enum rather than a string, with variants Table, Array and Field

impl<'a> NamedTable<'a> {
    pub fn as_struct_name(&self) -> String {
        struct_name_for_table_path(&self.path)
    }

    pub fn field_type_of(&self, name: &str) -> Result<String, String> {
        let value = self.field_value_named(name)?;
        let table_path = &self.path;
        self::field_type_of(&format!("{table_path}\x1d{name}"), value)
    }

    fn field_value_named(&self, name: &str) -> Result<&Value, String> {
        self.as_map
            .get(name)
            .ok_or_else(|| format!("Field not present in table; field={name}, table={}", human_readable_path(&self.path)))
    }

    pub fn field_name_of(&self, name: &str) -> Result<String, String> {
        _ = self.field_value_named(name)?;
        Ok(self::field_name_for_value_name(name))
    }
}

fn struct_name_for_table_path(path: &str) -> String {
    static REPLACEMENTS: OnceLock<Regex> = OnceLock::new();
    let regex = REPLACEMENTS.get_or_init(|| Regex::new(&format!("{}|{}|{}|{}|{}",
        "^([a-z])",
        "[-_. \t:;/#~@+$!\\|]([a-z])",
        "(\x1d[a-z])",
        "(\x1d)",
        "[^a-z0-9]+")).unwrap());

    let lowercased_path = path.to_lowercase();
    let rust_friendly_path = regex.replace_all(&lowercased_path, |captures: &Captures| {
        let captures = captures.iter().skip(1).filter(|x| x.is_some()).collect::<Vec<_>>();
        if captures.len() == 1 {
            captures[0].unwrap().as_str().replace('\x1d', "_").to_uppercase()
        } else {
            "".to_string()
        }
    });

    format!("Config_{}", rust_friendly_path.trim_matches('_')).trim_end_matches('_').to_string()
}

fn field_type_of(path: &str, value: &Value) -> Result<String, String> {
    match value {
        Value::Array(elements) if elements.is_empty() => Ok("[isize; 0]".to_string()),
        Value::Array(elements) => Ok(format!("[{}; {}]", field_type_of(&format!("{path}\x1d0"), &elements[0])?, elements.len())),
        Value::Boolean(_) => Ok("bool".to_string()),
        Value::Float(_) => Ok("f64".to_string()),
        Value::Integer(_) => Ok("i64".to_string()),
        Value::String(_) => Ok("&'static str".to_string()),
        Value::Table(_) => Ok(struct_name_for_table_path(path)),
        _ => Err(format!("Unknown type of config value; path={}", human_readable_path(path)))
    }
}

fn human_readable_path(path: &str) -> String {
    path.replace('\x1d', " / ")
}

fn field_name_for_value_name(name: &str) -> String {
    static REPLACEMENTS: OnceLock<Regex> = OnceLock::new();
    let regex = REPLACEMENTS.get_or_init(|| Regex::new("[^-_A-Z0-9]*([-_A-Z0-9]*)[^-_A-Z0-9]*").unwrap());

    let uppercased_name = name.to_uppercase();
    let rust_friendly_name = regex.replace_all(&uppercased_name, |captures: &Captures| {
        captures[1].replace('-', "_")
    }).to_string();

    if rust_friendly_name.starts_with(|ch| char::is_digit(ch, 10)) {
        format!("_{rust_friendly_name}")
    } else {
        rust_friendly_name
    }
}

fn field_value_of(path: &str, value: &Value) -> Result<TokenStream, String> {
    match value {
        Value::Array(elements) => {
            let tokens = elements.iter().map(|element| field_value_of(&format!("{path}\x1d0"), element)).collect::<Result<Vec<_>, _>>()?;
            Ok(quote! {
                [ #( #tokens, )* ]
            })
        },
        Value::Boolean(value) => Ok(quote! { #value }),
        Value::Float(value) => Ok(quote! { #value }),
        Value::Integer(value) => Ok(quote! { #value }),
        Value::String(value) => Ok(quote! { #value }),
        Value::Table(table) => {
            let struct_type = field_type_of(path, value)?.parse::<TokenStream>().map_err(|err| err.to_string())?;
            let tokens = table.iter().map(|(name, value)| {
                let value = field_value_of(&format!("{path}\x1d{name}"), value)?;
                let name = Ident::new(&field_name_for_value_name(name), Span::call_site());
                Ok(quote! {
                    #name: #value
                })
            }).collect::<Result<Vec<_>, String>>()?;

            Ok(quote! {
                #struct_type { #( #tokens, )* }
            })
        },
        _ => Err(format!("Unknown type of config value; path={}", human_readable_path(path)))
    }
}

#[derive(Debug)]
struct ParsedConfig<'a> {
    all_tables: Vec<NamedTable<'a>>
}

impl<'a> ParsedConfig<'a> {
    pub fn new(config: &'a Value) -> Self {
        Self {
            all_tables: Self::all_tables_from(config)
        }
    }

    fn all_tables_from(config: &'a Value) -> Vec<NamedTable<'a>> {
        let mut all_tables = vec![];
        Self::add_all_tables_into(&mut all_tables, "".to_string(), config);
        all_tables
    }

    fn add_all_tables_into(all_tables: &mut Vec<NamedTable<'a>>, path: String, value: &'a Value) {
        match value {
            Value::Table(nested) => {
                let path_prefix = format!("{path}\x1d");
                all_tables.push(NamedTable { path, as_value: value, as_map: value.as_table().unwrap() });
                nested.iter().for_each(|(key, value)| Self::add_all_tables_into(all_tables, format!("{path_prefix}{key}"), value));
            },
            Value::Array(nested) if !nested.is_empty() => {
                let path_prefix_first_element = format!("{path}\x1d0");
                Self::add_all_tables_into(all_tables, path_prefix_first_element, &nested[0]);
            },
            _ => { }
        };
    }

    pub fn tokens_for_struct_declarations(&self, struct_visibility: &Visibility) -> Result<TokenStream, String> {
        let structs = self.all_tables.iter().map(|table| self.tokens_for_struct_declaration(struct_visibility, table)).collect::<Result<Vec<_>, _>>()?;
        Ok(quote! {
            #( #structs )*
        })
    }

    fn tokens_for_struct_declaration(&self, struct_visibility: &Visibility, table: &NamedTable) -> Result<TokenStream, String> {
        let struct_name = Ident::new(&table.as_struct_name(), Span::call_site());
        let fields = table.as_map.keys().map(|name| Self::tokens_for_struct_field_declaration(table, name)).collect::<Result<Vec<TokenStream>, String>>()?;
        Ok(quote! {
            #[derive(Debug, PartialEq)]
            #struct_visibility struct #struct_name {
                #( #fields, )*
            }
        })
    }

    fn tokens_for_struct_field_declaration(table: &NamedTable, name: &str) -> Result<TokenStream, String> {
        let field_type = table.field_type_of(name)?.parse::<TokenStream>().map_err(|err| err.to_string())?;
        let field_name = Ident::new(&table.field_name_of(name)?, Span::call_site());
        Ok(quote! {
            pub #field_name: #field_type
        })
    }

    pub fn tokens_for_field_values(&self) -> Result<TokenStream, String> {
        if let Some(root) = self.all_tables.first() {
            field_value_of(&root.path, root.as_value)
        } else {
            Ok(quote! { { /* no config */ } })
        }
    }
}

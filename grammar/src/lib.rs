extern crate rustling;
extern crate rustling_ontology_values;
extern crate rustling_ontology_en as en;

use std::result;

macro_rules! lang_enum {
    ([$($lang:ident),*]) => {
        /// Enumerates all language supported for the general purpose ontology.
        #[derive(Copy,Clone,Debug,PartialEq, Eq)]
        pub enum Lang {
            $( $lang, )*
        }

        impl Lang {
            pub fn all() -> Vec<Lang> {
                vec![
                    $( Lang::$lang, )*
                ]
            }
        }

        impl std::str::FromStr for Lang {
            type Err = String;
            fn from_str(it: &str) -> result::Result<Lang, Self::Err> {
                match &*it.to_uppercase() {
                    $( stringify!($lang) => Ok(Lang::$lang),  )*
                    _ => Err(format!("Unknown language {}", it)),
                }
            }
        }

        impl ::std::string::ToString for Lang {
            fn to_string(&self) -> String {
                match self {
                    $( &Lang::$lang => stringify!($lang).to_string(),)*
                }
            }
        }

    }
}

lang_enum!([EN]);

/// Obtain rules for a given language.
pub fn rules(lang: Lang) -> ::rustling::RustlingResult<::rustling::RuleSet<rustling_ontology_values::Dimension>> {
    match lang {
        Lang::EN => en::rule_set(),
    }
}

/// Obtain dimensions for a given language.
pub fn dims(lang: Lang) -> Vec<rustling_ontology_values::DimensionKind> {
    match lang {
        Lang::EN => en::dims(),
    }
}

/// Obtain examples for a given language.
pub fn examples(lang: Lang) -> Vec<::rustling::train::Example<rustling_ontology_values::Dimension>> {
    match lang {
        Lang::EN => en::examples(),
    }
}

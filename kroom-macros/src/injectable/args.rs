use std::str::FromStr;

use proc_macro::TokenStream;
use syn::{LitStr, Type, meta, parse::Parser};

pub enum Scope {
    Singleton,
    Transient,
    Scoped
}
impl FromStr for Scope {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "singleton" => Ok(Scope::Singleton),
            "transient" => Ok(Scope::Transient),
            "scoped" => Ok(Scope::Scoped),
            _ => Err(format!("{} is not a valid Scope",s)),
        }
    }
}

#[derive(Default)]
pub struct MacroArgs {
    pub target_type: Option<Type>,
    pub scope: Option<Scope>
}

pub fn parse_macro_args(
    attr: TokenStream,
) -> Result<MacroArgs, syn::Error> {
    let mut args = MacroArgs::default();
    let parser = meta::parser(|metadata| {
        if metadata.path.is_ident("scope") {
            let value: String = metadata
                .value()?
                .parse::<LitStr>()?
                .value();
            let scope = Scope::from_str(&value)
                .map_err(|err| metadata.error(err))?;
            args.scope = Some(scope);
            Ok(())
        } else if metadata.path.is_ident("type") {
            let target_type: Type = metadata
                .value()?
                .parse()?;
            args.target_type = Some(target_type);
            Ok(())
        } else {
            Err(metadata.error("Unsupported macro argument"))
        }
    });
    parser.parse(attr.into())?;
    Ok(args)
}
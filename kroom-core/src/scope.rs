use std::str::FromStr;

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
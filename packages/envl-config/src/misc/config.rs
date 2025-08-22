#[derive(Debug, Clone)]
pub struct Settings {}

#[derive(Debug, Clone)]
pub struct Vars {}

#[derive(Debug, Clone)]
pub struct Config {
    pub settings: Settings,
    pub vars: Vars,
}

use config::AddToCMake;

pub mod config;
pub use cmake;

impl<'a> From<&'a config::Config> for cmake::Config {
    fn from(cfg: &config::Config) -> Self {
        let mut cmake_cfg = cmake::Config::new(&cfg.path);
        cfg.add_to_cmake(&mut cmake_cfg);
        cmake_cfg
    }
}

use std::fs::File;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
pub struct Settings {
    bump_limit: i32,
}

impl Settings {
    pub fn load() -> Result<Settings, Box<::std::error::Error>> {
        let mut settings = String::default();
        File::open("./board_settings.toml").and_then(|mut x| x.read_to_string(&mut settings))?;
        let settings = toml::from_str::<Settings>(&settings)?;
        Ok(settings)
    }
    pub fn bump_limit(&self) -> i32 {
        self.bump_limit
    }
}

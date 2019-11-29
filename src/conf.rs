use config;

pub struct Config {
    url: String,
    login: String,
    pass: String,
}

impl Config {
    pub fn get() -> Config {
        let conf_path = config::File::from_str("/etc/zstat.ini", config::FileFormat::Ini);
        let cf = config::Config::new();
        let login = cf.get_str("login").unwrap_or("admin".to_string());
        let pass = cf.get_str("pass").unwrap_or("password".to_string());
        let url = cf.get_str("url").unwrap_or("localhost".to_string());

        Config {
            url,
            login,
            pass,
        }
    }
}

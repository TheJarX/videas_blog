use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct NavbarLink {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub headline: String,
    pub short_name: String,
    pub picture_url: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct Site {
    pub title: String,
    pub base_url: String,
    pub cover_path: String,
    pub description: String,
    pub navbar_links: Vec<NavbarLink>
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub site: Site,
    pub author: Author,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_str = fs::read_to_string("blog_config.toml")
        .expect("Failed to read config file");
    toml::from_str(&config_str)
        .expect("Failed to parse config file")
});

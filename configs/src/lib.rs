use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Configs {
    #[serde(default)]
    pub rooms: Vec<i64>,
    pub streamers: Vec<Streamer>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Streamer {
    pub id: usize,
    pub nickname: String,
    pub username: String,
    pub bilibili_link: String,
    pub room_id: i64,
    pub avatar: String,
    pub small_avatar: String,
    pub description: String,
}

impl Default for Configs {
    fn default() -> Self {
        let file_path = "./configs.toml";
        let toml_string = std::fs::read_to_string(file_path).unwrap();
        let mut configs: Configs = toml::from_str(&toml_string).unwrap();
        configs.rooms = configs.streamers.iter().map(|x| x.room_id).collect();
        configs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let config = Configs::default();
        println!("{:?}", config);
    }
}

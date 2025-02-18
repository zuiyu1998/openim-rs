use config::Config;
use serde::de::DeserializeOwned;

pub fn load_config_with_file_name<T: DeserializeOwned>(file: &str) -> T {
    let config = Config::builder()
        .add_source(config::File::with_name(file))
        .build()
        .unwrap();

    config
        .try_deserialize()
        .expect("Config deserialize failed.")
}

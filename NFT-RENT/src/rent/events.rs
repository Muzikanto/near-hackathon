use near_sdk::{env};

pub(crate) fn log_event(method: &String, data: String) {
  env::log_str(&format!("{}{}{}{}{}", "EVENT_JSON:{\"standard\":\"mfight_rent\",\"version\":\"1.0.0\",\"event\":\"", method.to_string(), "\",\"data\": [", data, "] }"));
}



use crate::LOG_DRAIN;

use serde::{Deserialize, Serialize};
use slog::info;


#[derive(Deserialize, Default, Debug, Serialize)]
#[serde(default)]
pub(crate) struct Settings {}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
            //this policy doesn't have settings
            Ok(())
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn validate_settings() -> Result<(), ()> {
        let settings = Settings {};

        assert!(settings.validate().is_ok());
        Ok(())
    }
}
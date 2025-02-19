use serde_derive::{Deserialize, Serialize};

const HUAK_REQUIRES: &str = "huak-core>=1.0.0";
const HUAK_BUILD_BACKEND: &str = "huak.core.build.api";

/// Build System data.
/// ```toml
/// [tool.build-system]
/// # ...
/// ```
#[derive(Serialize, Deserialize)]
pub(crate) struct BuildSystem {
    pub(crate) requires: Vec<String>,
    #[serde(rename = "build-backend")]
    pub(crate) backend: String,
}

impl Default for BuildSystem {
    fn default() -> BuildSystem {
        BuildSystem {
            requires: vec![HUAK_REQUIRES.to_string()],
            backend: HUAK_BUILD_BACKEND.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn build_system() {
        let requires = vec![];
        let backend = "".to_string();
        let string = r#"requires = []
build-backend = ""
"#;

        let data = BuildSystem {
            requires: requires.clone(),
            backend: backend.clone(),
        };

        assert_eq!(data.requires, requires);
        assert_eq!(data.backend, backend);
        assert_eq!(toml::to_string(&data).unwrap(), string);
    }
}

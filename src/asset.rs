use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Asset {
    name: String,
    value: usize,
    hash: String,
    owner: Option<String>,
}

impl Display for Asset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let owner = if let Some(val) = self.get_owner() {
            val
        } else {
            "Fresh out of mint"
        };
        write!(
            f,
            "Hash:{}\nName: {}\nValue: {}\nOwner: {}",
            self.get_hash(),
            self.get_name(),
            self.get_value(),
            owner
        )
    }
}

impl Asset {
    pub fn new(name: &str, value: usize) -> Self {
        use super::utils::hasher;
        let hash = hasher(&format!("{}{}", &name, &value));
        Asset {
            name: name.into(),
            value,
            owner: None,
            hash,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_owner(&mut self, owner: &str) {
        self.owner = Some(owner.into());
    }

    pub fn get_owner(&self) -> &Option<String> {
        &self.owner
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mint_asset_works() {
        let mut asset = Asset::new("Delhi", 100);
        assert_eq!(asset.get_name(), "Delhi");
        assert_eq!(asset.get_owner(), &None);
        assert_eq!(asset.get_value(), 100);

        let new_owner = "Me".to_string();
        asset.set_owner(&new_owner);
        assert_eq!(asset.get_owner(), &Some(new_owner));

        assert_eq!(
            asset.get_hash(),
            "C0FBFAB8CDB750FE28D3AD79E401094B889D969CBC9A31C140859EA2E96592E2"
        );
    }
}

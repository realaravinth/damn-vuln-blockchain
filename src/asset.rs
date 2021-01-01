use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Asset {
    name: String,
    value: usize,
    owner: Option<String>,
}

impl Asset {
    pub fn new(name: &str, value: usize) -> Self {
        Asset {
            name: name.into(),
            value,
            owner: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
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
        assert_eq!(asset.get_owner(), &Some(new_owner))
    }
}

use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct BlockBuilder {
    prev: String,
    tx: String,
    rx: String,
}

impl BlockBuilder {
    pub fn set_prev(&mut self, prev: &Block) -> &mut Self {
        self.prev = prev.get_hash().into();
        self
    }

    pub fn set_rx(&mut self, rx: &str) -> &mut Self {
        self.rx = rx.into();
        self
    }

    pub fn set_tx(&mut self, tx: &str) -> &mut Self {
        self.tx = tx.into();
        self
    }

    pub fn build(&mut self) -> Block {
        use crate::utils::hasher;
        if self.prev.is_empty() || self.rx.is_empty() || self.tx.is_empty() {
            panic!("Can't create block, one or more fields are empty");
        } else {
            let hash = hasher(&format!("{}{}{}", self.prev, self.rx, self.tx));
            Block {
                prev: Some(self.prev.to_owned()),
                tx: Some(self.tx.to_owned()),
                rx: Some(self.rx.to_owned()),
                hash,
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Block {
    prev: Option<String>,
    hash: String,
    tx: Option<String>,
    rx: Option<String>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Previous Block: {}\nHash: {}\nSender: {}\nReceiver: {}\n",
            self.get_prev(),
            self.get_hash(),
            self.get_rx(),
            self.get_tx()
        )
    }
}

impl Block {
    /// First block of a blockchain
    pub fn genesis() -> Self {
        use std::iter;

        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        use crate::utils::hasher;

        let mut rng = thread_rng();
        let content: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(7)
            .collect();
        let hash = hasher(&content);
        Block {
            prev: None,
            tx: None,
            rx: None,
            hash,
        }
    }

    pub fn get_prev(&self) -> &str {
        match &self.prev {
            Some(val) => val,
            None => "Genesis block",
        }
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_rx(&self) -> &str {
        match &self.rx {
            Some(val) => val,
            None => "Genesis block",
        }
    }

    pub fn get_tx(&self) -> &str {
        match &self.tx {
            Some(val) => val,
            None => "Genesis block",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn block_works() {
        let prev = Block::genesis();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .build();
        assert_eq!(block.get_tx(), "Me");
        assert_eq!(block.get_rx(), "You");
    }
}

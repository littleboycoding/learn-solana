use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccountData {
    pub counter: u8,
}

impl AccountData {
    pub fn inc(&mut self) {
        self.counter = self.counter.checked_add(1).unwrap();
    }

    pub fn dec(&mut self) {
        self.counter = self.counter.checked_sub(1).unwrap();
    }
}

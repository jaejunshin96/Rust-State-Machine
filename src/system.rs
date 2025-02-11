use std:: collections::BTreeMap;
use std::ops::AddAssign;
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + CheckedAdd + CheckedSub + One + Copy + AddAssign;
	type Nonce: Ord + Zero + One + Clone + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>
}

impl <T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self {
			block_number: T::BlockNumber::zero(),
			nonce: BTreeMap::new()
		}
	}

	pub fn block_number(&mut self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
		//self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = self.nonce.get(who).cloned().unwrap_or(T::Nonce::zero());
		let new_nonce = nonce.checked_add(&T::Nonce::one()).unwrap();
		self.nonce.insert(who.clone(), new_nonce);
	}

	pub fn get_nonce(&mut self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::one())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
    impl super::Config for TestConfig {
		type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let foo = String::from("foo");
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_nonce(&foo);
		assert_eq!(system.get_nonce(&foo), 1);
	}
}

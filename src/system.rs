use std:: collections::BTreeMap;
use std::ops::AddAssign;
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>
}

impl <AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
	AccountId: Ord + Clone,
	BlockNumber: Zero + CheckedAdd + CheckedSub + One + Copy + AddAssign,
	Nonce: Ord + Zero + One + Clone + CheckedAdd + Copy,
{
	pub fn new() -> Self {
		Self {
			block_number: BlockNumber::zero(),
			nonce: BTreeMap::new()
		}
	}

	pub fn block_number(&mut self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one();
		//self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce = self.nonce.get(who).cloned().unwrap_or(Nonce::zero());
		let new_nonce = nonce.checked_add(&Nonce::one()).unwrap();
		self.nonce.insert(who.clone(), new_nonce);
	}

	pub fn get_nonce(&mut self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&Nonce::one())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let foo = String::from("foo");
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_nonce(&foo);
		assert_eq!(system.get_nonce(&foo), 1);
	}
}

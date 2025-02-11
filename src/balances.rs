use std::{clone, collections::BTreeMap};
use num::traits::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	balances: BTreeMap<AccountId, Balance>,
}

impl <AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: Zero + CheckedAdd + CheckedSub + Copy,
{
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new()
		}
	}

	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: AccountId,
		to: AccountId,
		amount: Balance
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance
			.checked_sub(&amount)
			.ok_or("Insufficient balance")?;
		let new_to_balance = to_balance
			.checked_add(&amount)
			.ok_or("Overflow balance")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance );

		Ok(())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<String, u128>::new();

		assert_eq!(balances.balance(&"foo".to_string()), 0);

		balances.set_balance(&"foo".to_string(), 100);


		assert_eq!(balances.balance(&"foo".to_string()), 100);
		assert_eq!(balances.balance(&"bar".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let foo = "foo".to_string();
		let bar = "bar".to_string();

		let mut balances = super::Pallet::<String, u128>::new();

		balances.set_balance(&foo, 100);
		balances.set_balance(&bar, 0);

		let _ = balances.transfer(foo.clone(), bar.clone(), 50);

		assert_eq!(balances.balance(&foo), 50);
		assert_eq!(balances.balance(&bar), 50);
	}
}

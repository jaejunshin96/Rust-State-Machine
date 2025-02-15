use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
	//type AccountId: Ord + Clone;
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T:: AccountId, T::Balance>,
}

impl <T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new()
		}
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

//pub enum Call<T: Config> {
//	Transfer { to: T::AccountId, amount: T::Balance },
//}

//impl<T: Config> crate::support::Dispatch for Pallet<T> {
//	type Caller = T::AccountId;
//	type Call = Call<T>;

//	fn dispatch(
//		&mut self,
//		caller: Self::Caller,
//		call: Self::Call,
//	) -> crate::support::DispatchResult {
//		/* TODO: use a `match` statement to route the `Call` to the appropriate pallet function. */
//		match call {
//			Call::Transfer { to, amount } => {
//				self.transfer(caller, to, amount)?;
//			},
//		}
//		Ok(())
//	}
//}

#[cfg(test)]
mod test {

	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
	}

	impl super::Config for TestConfig {
		//type AccountId = String;
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"foo".to_string()), 0);

		balances.set_balance(&"foo".to_string(), 100);


		assert_eq!(balances.balance(&"foo".to_string()), 100);
		assert_eq!(balances.balance(&"bar".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let foo = "foo".to_string();
		let bar = "bar".to_string();

		let mut balances = super::Pallet::<TestConfig>::new();

		balances.set_balance(&foo, 100);
		balances.set_balance(&bar, 0);

		let _ = balances.transfer(foo.clone(), bar.clone(), 50);

		assert_eq!(balances.balance(&foo), 50);
		assert_eq!(balances.balance(&bar), 50);
	}
}

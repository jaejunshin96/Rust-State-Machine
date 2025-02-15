mod balances;
mod system;
mod support;
mod proof_of_existence;

use std::vec;

use crate::support::Dispatch;

mod types {
	use crate::support;

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

//pub enum RuntimeCall {
//	Balances(balances::Call<Runtime>),
//	ProofOfExistence(proof_of_existence::Call<Runtime>),
//}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	//type AccountId = String;
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

//impl Runtime {
//	fn new() -> Self {
//		Self {
//			system: system::Pallet::new(),
//			balances: balances::Pallet::new(),
//			proof_of_existence: proof_of_existence::Pallet::new()
//		}
//	}
//	// Execute a block of extrinsics. Increments the block number.
//	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
//		self.system.inc_block_number();
//		if block.header.block_number != self.system.block_number() {
//			return Err("Block numbers don't match");
//		}
//		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
//			// do stuff with `caller` and `call`
//			self.system.inc_nonce(&caller);
//			let _res = self.dispatch(caller, call).map_err(|e| {
//				eprintln!(
//					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
//					block.header.block_number, i, e
//				)
//			});
//		}
//		Ok(())
//	}
//}

//impl crate::support::Dispatch for Runtime {
//	type Caller = <Runtime as system::Config>::AccountId;
//	type Call = RuntimeCall;
//	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
//	//
//	// Dispatch allows us to identify which underlying module call we want to execute.
//	// Note that we extract the `caller` from the extrinsic, and use that information
//	// to determine who we are executing the call on behalf of.
//	fn dispatch(
//		&mut self,
//		caller: Self::Caller,
//		runtime_call: Self::Call,
//	) -> support::DispatchResult {
//		match runtime_call {
//			RuntimeCall::Balances(call) => {
//				self.balances.dispatch(caller, call)?;
//			},
//			RuntimeCall::ProofOfExistence(call) => {
//				self.proof_of_existence.dispatch(caller, call)?;
//			}
//		}
//		Ok(())
//	}
//}

fn main() {
    println!("Hello, rust!");
	let mut runtime = Runtime::new();
	let jae = String::from("jae");
	let foo = String::from("foo");
	let bar = String::from("bar");

	runtime.balances.set_balance(&jae, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: jae.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: (foo.clone()), amount: (11) })
			},
			support::Extrinsic {
				caller: jae.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: (bar.clone()), amount: (11) })
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: jae.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "jaeClaim" })
			},
			support::Extrinsic {
				caller: foo.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "fooClaim" })
			},
			support::Extrinsic {
				caller: jae.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim { claim: "randomClaim" })
			},
		]
	};

	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");

	print!("{:#?}", runtime);
}

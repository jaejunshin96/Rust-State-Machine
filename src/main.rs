mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = String;
	type BlockNumber = u32;
	type Nonce = u32;
}

impl balances::Config for Runtime {
	//type AccountId = String;
	type Balance = u128;
}

impl Runtime {
	fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new()
		}
	}
}

fn main() {
    println!("Hello, rust!");
	let mut runtime = Runtime::new();
	let jae = String::from("jae");
	let foo = String::from("foo");
	let bar = String::from("bar");

	runtime.balances.set_balance(&jae, 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&jae);
	let result = runtime.balances.transfer(jae.clone(), foo.clone(), 30);
	match result {
		Ok(_) => {
			println!("Transfer successful");
		},
		Err(e) => {
			println!("Error: {}", e);
		}
	}

	runtime.system.inc_nonce(&jae);
	let _ = runtime.balances
		.transfer(jae.clone(), bar.clone() , 20)
		.map_err(|e| println!("Error: {}", e));

	println!("{:#?}", runtime);
}

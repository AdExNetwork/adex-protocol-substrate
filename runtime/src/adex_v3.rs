use parity_codec::Encode;
use srml_support::{StorageValue, dispatch::Result};
use {balances, system::{self, ensure_signed}};
use runtime_primitives::traits::Member;

pub trait Trait: balances::Trait {}

#[derive(Debug)]
pub struct Channel<AccountId, Balance> {
    creator: AccountId,
    deposit: Balance,
    valid_until: u64,
    validators: Vec<AccountId>,
    spec: Vec<u8>,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // @TODO should valid_until be some sort of substrate-specific lifetime value?
		fn channel_start(origin, deposit: T::Balance,  valid_until: u64, validators: Vec<T::AccountId>, spec: Vec<u8>) -> Result {
            // @TODO: move this to impl<T: Trait> Module<T>
            let sender = ensure_signed(origin)?;

            let channel = Channel {
                creator: sender,
                deposit: deposit,
                valid_until: valid_until,
                validators: validators,
                spec: spec,
            };

            println!("{:?}", channel);

		    //<balances::Module<T>>::decrease_free_balance(&sender, payment)?;
		    Ok(())
        }
		//fn channel_finalize(origin, channel: Commitment<T::AccountId, T::Balance>) -> Result;
	}

    fn channel_withdraw_expired(origin, deposit: T::Balance,  valid_until: u64, validators: Vec<T::AccountId>, spec: Vec<u8>) -> Result {
    }

    //  bytes32 stateRoot, bytes32[3][] memory signatures, bytes32[] memory proof, uint amountInTree
    fn channel_withdraw(origin, deposit: T::Balance,  valid_until: u64, validators: Vec<T::AccountId>, spec: Vec<u8>) -> Result {
    }
}

decl_storage! {
	trait Store for Module<T: Trait> as AdExV3 {
		Payment get(payment) config(): Option<T::Balance>;
        // @TODO Balance should carry multiple tokens
        // @TODO system to clean-up old channels
	}
}

impl<T: Trait> Module<T> {
    //fn channel_finalize(_: T::Origin, channel: Commitment<T::AccountId, T::Balance>) -> Result {
    //    Ok(())
    //}
}


#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*,sp_runtime::traits::{AtLeast32BitUnsigned,Saturating, Zero, CheckedAdd, CheckedSub,}, RuntimeDebug,
	};
	use frame_system::pallet_prelude::{*, OriginFor};
	const STORAGE_VERSION: frame_support::traits::StorageVersion =
		frame_support::traits::StorageVersion::new(1);

		#[pallet::pallet]
		#[pallet::without_storage_info]
		#[pallet::storage_version(STORAGE_VERSION)]
		pub struct Pallet<T, I=()>(PhantomData<(T, I)>);
	

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		type RuntimeEvent: From<Event<Self,I>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	}
	#[derive(Clone, Encode, Decode, Eq, PartialEq,TypeInfo, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct MetaData<AccountId, Balance> {
		issuance:Balance,
		minter: AccountId,
		burner:AccountId,
	}



    #[pallet::storage]
	#[pallet::getter(fn meta_data)]
	pub type MetaDataStore<T: Config<I>, I: 'static = ()> = StorageValue<_,MetaData<T::AccountId,T::Balance> ,OptionQuery>;

	
	#[pallet::storage]
	#[pallet::getter(fn account_balance)]
	pub(super) type Accounts<T: Config<I>, I: 'static = ()> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

		
	// Declare `admin` as type `T::AccountId`.
	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()>{
		pub phantom: PhantomData<I>,
		pub admin: Option<T::AccountId>
	}


    #[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			if let Some(ref admin) = self.admin {
				MetaDataStore::<T,I>::put(MetaData{
					issuance:Zero::zero(),
					minter: admin.clone(),
					burner:admin.clone(),
				});
		}}
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		Transfered(T::AccountId, T::AccountId, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// An account would go below the minimum balance if the operation were executed.
		BelowMinBalance,
		// The origin account does not have the required permission for the operation.
		NoPermission,
		/// An operation would lead to an overflow.
		Overflow,
		/// An operation would lead to an underflow.
		Underflow,
		/// Cannot burn the balance of a non-existent account.
		CannotBurnEmpty,
		/// There is not enough balance in the sender's account for the transfer.
		InsufficientBalance,
	}
	
	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balance,
			//ride Id 
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			Accounts::<T,I>::try_mutate(&signer, |bal| -> DispatchResult {
				let new_bal = bal.checked_sub(&amount).ok_or(Error::<T,I>::InsufficientBalance)?;
				
				*bal = new_bal;
				Ok(())
			})?;

			Accounts::<T,I>::try_mutate(&to, |rec_bal| -> DispatchResult {
				let new_bal = rec_bal.saturating_add(amount);
				
				*rec_bal = new_bal;
				Ok(())
			})?;	
			Self::deposit_event(Event::<T,I>::Transfered(signer, to, amount));		
			Ok(Pays::No.into())
		}


		
		// mint 
		//transfer 


	}
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		fn mint_into(who: &T::AccountId, amount: T::Balance)->bool {
			Accounts::<T,I>::mutate(&who, |bal| {
				let created = bal == &Zero::zero();
				
				// meta.issuance = meta.issuance.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
				// fine because we check the issuance for overflow before minting and transfers
				// don't change the issuance
				*bal = bal.saturating_add(amount);
				created
			})
		}
	}


}

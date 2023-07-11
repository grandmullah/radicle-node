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
	use frame_support::{pallet_prelude::*,sp_runtime::traits::{AtLeast32BitUnsigned, Zero}, RuntimeDebug,
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
	pub enum Event<T: Config<I>, I: 'static = ()> {

	}

	// #[pallet::error]
	
	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn accept_ride(
			origin: OriginFor<T>,
			//ride Id 
		) -> DispatchResultWithPostInfo {
			let _signer = ensure_signed(origin)?;
			
			Ok(Pays::No.into())
		}


		
		// mint 


	}
	impl<T: Config<I>, I: 'static> Pallet<T, I> {

	}



}
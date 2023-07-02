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
	use frame_support::{pallet_prelude::*,sp_runtime::traits::{Zero,Saturating,AtLeast32BitUnsigned,Member}};
	use frame_system::pallet_prelude::{*, OriginFor};


	
	
	use frame_support::sp_runtime::BoundedVec;
	
	use scale_info::TypeInfo;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct  Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		#[pallet::constant]
		type MaxIdLengthBytes: Get<u32>;
		
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}

	pub type Name<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;
	pub type PhoneNumber<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;

	
	#[derive(Eq, PartialEq, Encode,Decode,Default, TypeInfo,MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct MetaData<AccountId, Balance> {
		issuance: Balance,
		minter: AccountId,
		burner: AccountId,
	}

	
	#[pallet::storage]
	#[pallet::getter(fn account)]
	pub(super) type Accounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;


	#[pallet::event]
	pub enum Event<T:Config> {

	}

	// #[pallet::error]
	
	#[pallet::call]
	impl <T:Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn accept_ride(
			origin: OriginFor<T>,
			//ride Id 
		) -> DispatchResultWithPostInfo {
			let _signer = ensure_signed(origin)?;
			
			Ok(Pays::No.into())
		}
		
		// accept ride 
		// request ride 
		// feedback
		// emit rewards 
		// get estimates 

	}

}
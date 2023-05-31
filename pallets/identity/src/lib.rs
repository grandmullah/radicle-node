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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::{*, OriginFor};


	
	
	use frame_support::sp_runtime::BoundedVec;
	
	use scale_info::TypeInfo;

	#[pallet::pallet]
	pub struct  Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		#[pallet::constant]
		type MaxIdLengthBytes: Get<u32>;
	}

	pub type Name<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;
	pub type PhoneNumber<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;

	
	#[derive(Eq, PartialEq, Encode,Decode,Default, TypeInfo,MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct User<T:Config> {
		name:Name<T>,
		verified:bool,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_user)]
	pub(super) type Identity<T :Config>  = StorageMap<_, Blake2_128Concat, PhoneNumber<T>, User<T>,OptionQuery>;


	#[pallet::event]
	pub enum Event<T:Config> {

	}

	// #[pallet::error]
	
	#[pallet::call]
	impl <T:Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_usr(
			origin: OriginFor<T>,
			phone_number:PhoneNumber<T>,
			name:Name<T>
		) -> DispatchResultWithPostInfo {
			let _signer = ensure_signed(origin)?;
			let verified:bool = false;
			let usr_to_store = User::<T> {
				name,
				verified
			};

			<Identity<T>>::insert(phone_number,usr_to_store);
			// make a offchain call 
			Ok(Pays::No.into())
		}
		
		

	}

}
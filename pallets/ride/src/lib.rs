#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*,sp_runtime::traits::{Zero,Saturating,AtLeast32BitUnsigned,Member}};
	use frame_system::{pallet_prelude::{*, OriginFor}};


	
	
	use frame_support::sp_runtime::BoundedVec;
	
	use scale_info::TypeInfo;
	use pallet_reward::RewardInterface;
	use pallet_identity;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct  Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		#[pallet::constant]
		type MaxIdLengthBytes: Get<u32>;

		type RewardCoin:RewardInterface<Self::AccountId,Self::Balance>;

	
		
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}

	pub type Name<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;
	pub type PhoneNumber<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;

	#[derive(Encode, Decode, Clone, PartialEq, Eq,TypeInfo, RuntimeDebug)]
	pub struct UberRide<AccountId> {
		pub rider: AccountId,
		pub distance: u32,
		pub duration: u32,
		pub fare: u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn ride_count)]
	pub(super) type UserRides<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		Request(T::AccountId, T::AccountId, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The cab index specified does not exist
		InvalidIndex,

		/// Caller is not the owner 
		InvalidOwner,

		///Driver not availble
		DriverNotAvailable,

		///no count
		NoCount
	}
	
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


		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn request_ride(
			origin: OriginFor<T>,
			driver:T::AccountId,
			//ride Id 
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;

			let driver_details = pallet_identity::Pallet::<T>::get_driver(&driver).ok_or(Error::<T>::DriverNotAvailable)?;
			
			
			let count = Self::ride_count(&driver).ok_or(Error::<T>::NoCount)?;
			// check if the driver is ready 
			// calculate ride 
			// emit request ride 
			Self::deposit_event(Event::<T>::Request(signer,driver,count));
			Ok(Pays::No.into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn calculate_ride_fare(
			origin: OriginFor<T>,
			distance: u32,
			duration: u32,
		) -> DispatchResultWithPostInfo {
			let base_fare = 1;
			let cost_per_mile = 2;
			let cost_per_minute: u32 = 1;
			let fare = base_fare + distance * cost_per_mile + duration * cost_per_minute;
			// do something with the calculated fare
			Ok(().into())
		}
		
		// set  base fare 
		// set 
		
		// accept ride 
		// request ride 
		// feedback
		// emit rewards 
		// get estimates 

	}

}
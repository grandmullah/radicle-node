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
	use pallet_sacco::SaccoInterface;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct  Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		#[pallet::constant]
		type MaxIdLengthBytes: Get<u32>;

		type RewardCoin:RewardInterface<Self::AccountId,Self::Balance>;

		type Sacco:SaccoInterface;
		
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}

	pub type BoundedS<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;
	pub type PhoneNumber<T> = BoundedVec<u8,<T as Config>::MaxIdLengthBytes >;

	#[derive(Encode, Decode, Clone, PartialEq, Eq,TypeInfo, RuntimeDebug)]
	pub struct Ride<AccountId> {
		pub rider: AccountId,
		pub distance: u32,
		pub duration: u32,
		pub fare: u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn ride_count)]
	pub(super) type UserRides<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Ride<T::AccountId>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn rides_count)]
	pub(super) type UserRidesCount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		RideAccepted(T::AccountId, T::AccountId, BoundedS<T>),
		RideCompleted(T::AccountId, T::AccountId, BoundedS<T>,u32),
		UserRewards(T::AccountId, T::AccountId, BoundedS<T>,u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The cab index specified does not exist
		InvalidIndex,

		///
		UnverifiedDriver,

		/// Caller is not the owner 
		InvalidOwner,

		///Driver not availble
		DriverNotAvailable,

		///no count
		NoCount,

		///
		RideAccepted
	}
	
	#[pallet::call]
	impl <T:Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn accept_ride(
			origin: OriginFor<T>,
			ride_id:BoundedS<T>,
			driver:T::AccountId,
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			let driver_details = pallet_identity::Pallet::<T>::get_driver(&driver).ok_or(Error::<T>::DriverNotAvailable);
			let details = driver_details.unwrap();
			ensure!(details.verified,Error::<T>::UnverifiedDriver );
			ensure!(details.cab > 0u32, Error::<T>::DriverNotAvailable);
			Self::deposit_event(Event::<T>::RideAccepted(signer,driver,ride_id));
			Ok(Pays::No.into())
		}


		#[pallet::call_index(1)]
		#[pallet::weight(0)]

		pub fn complete_ride(
			origin: OriginFor<T>,
			ride_id:BoundedS<T>,
			distance: u32,
			duration: u32,
			rate:u32,
			rider:T::AccountId
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			let fare = Self::calculate_ride_fare(distance, duration);
			let reward = T::Sacco::get_rewards();
			let  rider_details = pallet_identity::Pallet::<T>::get_user(&rider).ok_or(Error::<T>::DriverNotAvailable);
			let  driver_details = pallet_identity::Pallet::<T>::get_driver(&signer).ok_or(Error::<T>::DriverNotAvailable);
			let mut details = rider_details.unwrap();
			let mut details_driver = driver_details.unwrap();
			let g = details.rating;

			details.rating = (g+rate)/2;
			details_driver.ride_count = (details_driver.ride_count)+1;
			pallet_identity::Identity::<T>::insert(&rider.clone(),&details);
			pallet_identity::Drivers::<T>::insert(&signer.clone(),&details_driver);

			
			T::RewardCoin::mint_to(&signer,&reward.into());
			Self::deposit_event(Event::<T>::UserRewards(signer.clone(), rider.clone(), ride_id.clone(),reward));
			Self::deposit_event(Event::<T>::RideCompleted(signer, rider, ride_id,fare));
			Ok(Pays::No.into())
		}
		#[pallet::call_index(2)]
		#[pallet::weight(0)]

		pub fn rider_feedback(
			origin: OriginFor<T>,
			ride_id:BoundedS<T>,
			rate:u32,
			driver:T::AccountId
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
		
			let reward = T::Sacco::get_rewards();
			let  driver_details = pallet_identity::Pallet::<T>::get_driver(&driver).ok_or(Error::<T>::DriverNotAvailable);
			let mut details = driver_details.unwrap();
			let g = details.rating;

			details.rating = (g+rate)/2;
			pallet_identity::Drivers::<T>::insert(&driver.clone(),&details);

			
			T::RewardCoin::mint_to(&signer,&reward.into());
			Self::deposit_event(Event::<T>::UserRewards(signer, driver, ride_id,reward));
			Ok(Pays::No.into())
		}

		// set  base fare 
		// set 
		
		// accept ride 
		// request ride 
		// feedback
		// emit rewards 
		// get estimates 

	}
	impl <T:Config> Pallet<T> {
		fn calculate_ride_fare(
			distance: u32,
			duration: u32,
		) ->u32 {
			let base_fare = T::Sacco::base_fare();
			let cost_per_mile = T::Sacco::cost_per_mile();
			let cost_per_minute=T::Sacco::cost_per_minute();
			let fare = base_fare + distance * cost_per_mile + duration * cost_per_minute;
			fare
		}
	}
	
}


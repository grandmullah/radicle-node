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
	use frame_support::{pallet_prelude::*,sp_runtime::traits::{Zero,Saturating}};
	use frame_system::{pallet_prelude::{*, OriginFor}, Account};


	
	
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

	
	#[derive(Eq, PartialEq, Encode,Decode,Default, TypeInfo,MaxEncodedLen,Clone)]
	#[scale_info(skip_type_params(T))]
	pub struct User<T:Config> {
		name:Name<T>,
		verified:bool,
		phone_number:PhoneNumber<T>,
		//role
		// address
		rating:u32
	}
	#[derive(Eq, PartialEq, Encode,Decode,Default, TypeInfo,MaxEncodedLen,Clone )]
	#[scale_info(skip_type_params(T))]
	pub struct Driver<T:Config> {
		name:Name<T>,
		verified:bool,
		phone_number:PhoneNumber<T>,
		cab:u32,
		rating:u32
	}

	#[derive(Eq, PartialEq, Encode,Decode, TypeInfo,MaxEncodedLen,Clone )]
	#[scale_info(skip_type_params(T))]
	pub struct Cab<T:Config> {
		plate:Name<T>,
		manufacture_year:u16,
		model:Name<T>,
		owner:T::AccountId,
		driver:T::AccountId,
		verified:bool,
		rating:u32,
	}


	#[pallet::storage]
	#[pallet::getter(fn get_user)]
	pub(super) type Identity<T :Config>  = StorageMap<_, Blake2_128Concat, T::AccountId, User<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_driver)]
	pub(super) type Drivers<T :Config>  = StorageMap<_, Blake2_128Concat, T::AccountId, Driver<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cab_count)]
	pub(super) type CabCount<T> = StorageValue<_, u32, ValueQuery>;

	

	#[pallet::storage]
	#[pallet::getter(fn get_cab)]
	pub(super) type CabDetails<T :Config>  = StorageMap<_, Blake2_128Concat, u32, Cab<T>, OptionQuery>;

	#[pallet::event]
	pub enum Event<T:Config> {

	}



	#[pallet::error]
	pub enum Error<T> {
		/// The cab index specified does not exist
		InvalidIndex,

		/// Caller is not the owner 
		InvalidOwner
	}
	
	#[pallet::call]
	impl <T:Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_usr(
			origin: OriginFor<T>,
			phone_number:PhoneNumber<T>,
			name:Name<T>
			
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			let verified:bool = false;
			let rating:u32 = 0;
			let usr_to_store = User::<T> {
				name,
				verified,
				phone_number,
				rating
			};

			<Identity<T>>::insert(&signer,usr_to_store);
			// make a offchain call 
			Ok(Pays::No.into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn add_cab(
			origin : OriginFor<T>,
			plate:Name<T>,
			manufacture_year:u16,
			model:Name<T>,
			driver:T::AccountId,
		) -> DispatchResultWithPostInfo{
			let owner = ensure_signed(origin)?;
			let count = Self::cab_count();

			// let mut  cab = Self::get_cab(count).ok_or(Error::<T>::InvalidIndex)?;
			<CabDetails::<T>>::insert(count+1,Cab{
				plate,
				manufacture_year,
				model,
				owner,
				driver,
				verified:false,
				rating:Zero::zero(),
			});

			<CabCount<T>>::put(count+1);
			Ok(Pays::No.into())
		}


		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn add_driver(
			origin: OriginFor<T>,
			phone_number:PhoneNumber<T>,
			name:Name<T>,
			cab:u32
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			let verified:bool = false;
			let rating:u32 = 0;
			

			<Drivers<T>>::insert(&account,Driver {
				name,
				verified,
				phone_number,
				cab,
				rating,
			});
			// make a offchain call 
			Ok(Pays::No.into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn assign_driver(
			origin: OriginFor<T>,
			cab:u32,
			driver:T::AccountId
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;

			let mut cab_details = Self::get_cab(cab).ok_or(Error::<T>::InvalidIndex)?;
			let mut driver_details = Self::get_driver(&driver).ok_or(Error::<T>::InvalidIndex)?;
			// cab.owner
			ensure!(account == cab_details.owner,<Error<T>>::InvalidOwner);
			let d = &driver.clone();
			cab_details.driver  = driver;
			driver_details.cab = cab;

	

			<Drivers<T>>::insert(&d,driver_details);
			<CabDetails::<T>>::insert(cab,cab_details);

			// <Drivers<T>>::insert(&account,Driver {
			// 	name,
			// 	verified,
			// 	phone_number,
			// 	cab,
			// 	rating,
			// });
			// make a offchain call 
			Ok(Pays::No.into())
			// emmit  transer driver 
		}
		
		// add cab
		// asider 
		//update driver
		//history of changes 
		//update changes
		

	}

}
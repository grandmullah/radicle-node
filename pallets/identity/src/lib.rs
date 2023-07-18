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
	use frame_support::{pallet_prelude::*,sp_runtime::traits::{Zero,Saturating}};
	use frame_system::{pallet_prelude::{*, OriginFor}, Account};
	use scale_info::prelude::string::String;

	
	
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
		pub name:Name<T>,
		pub verified:bool,
		pub phone_number:PhoneNumber<T>,
		//role
		// address
		pub rating:u32
	}
	#[derive(Eq, PartialEq, Encode,Decode,Default, TypeInfo,MaxEncodedLen,Clone )]
	#[scale_info(skip_type_params(T))]
	pub struct Driver<T:Config> {
		pub name:Name<T>,
		pub verified:bool,
		pub phone_number:PhoneNumber<T>,
		pub ride_count:u32,
		pub cab:u32,
		pub rating:u32
	}

	#[derive(Eq, PartialEq, Encode,Decode, TypeInfo,MaxEncodedLen,Clone )]
	#[scale_info(skip_type_params(T))]
	pub struct Cab<T:Config> {
		pub plate:Name<T>,
		pub manufacture_year:u16,
		model:Name<T>,
		owner:T::AccountId,
		driver:T::AccountId,
		verified:bool,
		rating:u32,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		/// Event emitted when a account  has been created.
		AccountCreated{who:T::AccountId,role:String},

		///Event emmitted when a cab is registered.
		CabAdded{who:T::AccountId,count:u32}
	}



	#[pallet::error]
	pub enum Error<T> {
		/// The cab index specified does not exist
		InvalidIndex,

		/// Caller is not the owner 
		InvalidOwner,

		/// driver does not exist
		DriverDoesNotExist
	}


	#[pallet::storage]
	#[pallet::getter(fn get_user)]
	pub type Identity<T :Config>  = StorageMap<_, Blake2_128Concat, T::AccountId, User<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_driver)]
	pub type Drivers<T :Config>  = StorageMap<_, Blake2_128Concat, T::AccountId, Driver<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cab_count)]
	pub(super) type CabCount<T> = StorageValue<_, u32, ValueQuery>;

	

	#[pallet::storage]
	#[pallet::getter(fn get_cab)]
	pub(super) type CabDetails<T :Config>  = StorageMap<_, Blake2_128Concat, u32, Cab<T>, OptionQuery>;

	
	
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
			let role  =  String::from("rider");
			let usr_to_store = User::<T> {
				name,
				verified,
				phone_number,
				rating
			};

			<Identity<T>>::insert(&signer,usr_to_store);
			// make a offchain call 
			Self::deposit_event(Event::<T>::AccountCreated {who:signer,role});
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
			let g = owner.clone();

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
			Self::deposit_event(Event::<T>::CabAdded{who:g,count:count+1});
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
			let ride_count:u32 = 0;			
			let role  =  String::from("driver");
			<Drivers<T>>::insert(&account,Driver {
				name,
				verified,
				phone_number,
				ride_count,
				cab,
				rating,
			});
			// make a offchain call 
			Self::deposit_event(Event::<T>::AccountCreated {who:account,role});
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
			let mut driver_details = Self::get_driver(&driver).ok_or(Error::<T>::DriverDoesNotExist)?;
			// cab.owner
			ensure!(account == cab_details.owner,<Error<T>>::InvalidOwner);
			let d = &driver.clone();
			cab_details.driver  = driver;
			driver_details.cab = cab;

	

			<Drivers::<T>>::insert(&d,driver_details);
			<CabDetails::<T>>::insert(cab,cab_details);


			Ok(Pays::No.into())
			// emmit  transer driver 
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn verify_cab(
			origin: OriginFor<T>,
			cab:u32,
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;

			let mut cab_details = Self::get_cab(cab).ok_or(Error::<T>::InvalidIndex)?;
			
			// cab.owner
			
			cab_details.verified = true;

	
			<CabDetails::<T>>::insert(cab,cab_details);


			Ok(Pays::No.into())
			// emmit  transer driver 
		}
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn verify_driver(
			origin: OriginFor<T>,
			driver:T::AccountId
		) -> DispatchResultWithPostInfo {
			let _account = ensure_signed(origin)?;

			let mut driver_details = Self::get_driver(&driver).ok_or(Error::<T>::InvalidIndex)?;
			
			// cab.owner
			
			driver_details.verified =  true;

	
			<Drivers::<T>>::insert(&driver,driver_details);


			Ok(Pays::No.into())
			// emmit  transer driver 
		}
		
		// add cab
		// asider 
		//update driver
		//history of changes 
		//update changes
		

	}

	pub trait  IdentityInterface<T:Config> {
		fn get_driver (who:&T::AccountId) -> Option<Driver<T>>;
	}
	impl<T: Config> IdentityInterface<T> for Pallet<T> {
		fn get_driver (who:&T::AccountId) -> Option<Driver<T>> {
			Self::get_driver(who)
		}
	}


}
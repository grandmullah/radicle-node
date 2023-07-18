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
	
	
	use scale_info::prelude::vec::Vec;

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

	#[derive(Encode, Decode, Clone, PartialEq, Eq,TypeInfo, RuntimeDebug)]
	pub struct Proposal {
		pub base_fare:u32,
		pub cost_per_mile:u32,
		pub cost_per_minute: u32,
		pub on:bool,
		pub votecount:u32,
		pub yee:u32,
		pub threshold:u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn proposal_index)]
	pub type Proposals<T> = StorageMap<_,  Blake2_128Concat, u32,Proposal, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn base_fare)]
	pub type BaseFare<T> = StorageValue<_, u32, ValueQuery>;
	#[pallet::storage]
	#[pallet::getter(fn reward_amount)]
	pub type Rewards<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cost_per_mile)]
	pub type CostPerMile<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cost_per_minute)]
	pub type CostPerMinute<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn completed_rides)]
	pub type PassedProposals<T: Config> = StorageValue<_, Vec<Proposal>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_members)]
	pub type Members<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		MemberAdded(T::AccountId),
		MemberRemoved(T::AccountId),
		REWARDAMOUNTSET(u32)
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


		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn set_proposal(
			origin: OriginFor<T>,
			base_fare:u32,
			cost_per_mile:u32,
			cost_per_minute: u32,
		) -> DispatchResultWithPostInfo {
			let _signer = ensure_signed(origin)?;
			let member_count = Members::<T>::get().len() as u32;
			let  threshold = member_count/2;

			let proposal:Proposal = Proposal {
				base_fare,
				cost_per_mile,
				cost_per_minute,
				on:true,
				votecount:0,
				yee:0,
				threshold,
			};


			Ok(Pays::No.into())
		}
		
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn add_member(
			origin: OriginFor<T>,
			
		) -> DispatchResultWithPostInfo {
			let new_member = ensure_signed(origin)?;
			Members::<T>::mutate(|members| members.push(new_member.clone()));
			// do something with the calculated fare

			Self::deposit_event(Event::<T>::MemberAdded(new_member));
			Ok(Pays::No.into())
		}
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn leave_membership(
			origin: OriginFor<T>,
			
		) -> DispatchResultWithPostInfo {
			let member = ensure_signed(origin)?;
			Members::<T>::mutate(|members| members.retain(|m| m != &member));
			// do something with the calculated fare

			Self::deposit_event(Event::<T>::MemberRemoved(member));
			Ok(Pays::No.into())
		}
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn setReward(
			origin: OriginFor<T>,
			reward:u32,
		) -> DispatchResultWithPostInfo {
			let member = ensure_signed(origin)?;
			<Rewards<T>>::put(reward);
			// do something with the calculated fare

			Self::deposit_event(Event::<T>::REWARDAMOUNTSET(reward));
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
	pub trait  SaccoInterface {
		fn base_fare()->u32;
		fn cost_per_mile()-> u32;
		fn cost_per_minute()->u32;
		fn get_rewards()->u32;

	}
	
	impl <T:Config>SaccoInterface for Pallet<T> {
		fn base_fare ()->u32 {
			Self::base_fare()
		}
		fn cost_per_mile()->u32{
			Self::cost_per_mile()
		}
		fn cost_per_minute()->u32{
			Self::cost_per_minute()
		}
		fn get_rewards()->u32{
			Self::reward_amount()
		}
	}

}
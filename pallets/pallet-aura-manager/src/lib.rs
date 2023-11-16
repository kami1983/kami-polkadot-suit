#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
pub use pallet_session::SessionManager;
pub type SessionIndex = u32;
use sp_std::vec::Vec;
use frame_support::sp_runtime;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::{
		RuntimeAppPublic,
	};
	use bound_vec_helper::BoundVecHelper;


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// The identifier type for an authority.
		// type AuthorityId: Member
		// + Parameter
		// + RuntimeAppPublic
		// + MaybeSerializeDeserialize
		// + MaxEncodedLen;
		/// A stable ID for a validator.
		type ValidatorId: Member
			+ Parameter
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TryFrom<Self::AccountId>;

		/// The maximum number of authorities that the pallet can hold.
		type MaxAuthorities: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub(super) type Validators<T: Config> =
	StorageValue<_, BoundedVec<T::ValidatorId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub authorities: Vec<T::ValidatorId>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// <Validators<T>>::put(&self.authorities.clone());
			let mut authorities = BoundedVec::<T::ValidatorId, T::MaxAuthorities>::default();
			for x in self.authorities.iter()  {
				authorities.try_push(x.clone()).unwrap();
			}
			<Validators<T>>::put(authorities);
		}
	}


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		///
		SetAuthorities{
			validators: Vec<T::ValidatorId>,
		},
		AddValidator{
			validator: T::ValidatorId,
		},
		RemoveValidator{
			validator: T::ValidatorId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		TooManyAuthorities,
		NotFoundValidator,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(1)]
		pub fn set_validator_list(origin: OriginFor<T>, validators: Vec<T::ValidatorId> ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			ensure_root(origin)?;

			let list = BoundedVec::<T::ValidatorId, T::MaxAuthorities>::create_on_vec(validators.clone());

			// Update storage.
			<Validators<T>>::put(list );

			// Emit an event.
			Self::deposit_event(Event::SetAuthorities { validators: validators.to_vec() });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(1)]
		pub fn add_validator_list(origin: OriginFor<T>, validator: T::ValidatorId ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			ensure_root(origin)?;
			// get old list
			let mut list = <Validators<T>>::get();
			list.try_push(validator.clone()).map_err(|_| Error::<T>::TooManyAuthorities)?;

			// Update storage.
			<Validators<T>>::put(list );

			// Emit an event.
			Self::deposit_event(Event::AddValidator { validator: validator });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(1)]
		pub fn remove_validator_list(origin: OriginFor<T>, validator: T::ValidatorId ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			ensure_root(origin)?;

			// Get old list
			let mut list = <Validators<T>>::get();
			// Check if the validator is in the list
			let index = list.iter().position(|x| *x == validator.clone()).ok_or(Error::<T>::NotFoundValidator)?;
			// Remove the validator from the list
			list.remove(index);

			// Update storage.
			<Validators<T>>::put(list );
			// Emit an event.
			Self::deposit_event(Event::RemoveValidator { validator: validator });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

/// In this implementation `new_session(session)` must be called before `end_session(session-1)`
/// i.e. the new session must be planned before the ending of the previous session.
///
/// Once the first new_session is planned, all session must start and then end in order, though
/// some session can lag in between the newest session planned and the latest session started.
impl<T: Config> SessionManager<T::ValidatorId> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
		Some(Validators::<T>::get().to_vec())
	}
	fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
		Some(Validators::<T>::get().to_vec())
	}
	fn start_session(start_index: SessionIndex) { }
	fn end_session(end_index: SessionIndex) { }
}

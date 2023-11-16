#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::transactional;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::BlockNumberProvider;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type BatchMintSize: Get<u8>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	pub type TypeCollectionDataLength = ConstU32<100>;
	pub type TypeBidLength = ConstU32<100>;

	pub type TypeNftCount = u64;
	pub type TypeBid = BoundedVec<u8, TypeBidLength>;
	pub type TypeSid = u64;
	pub type TypeCount = u64;

	pub type TypeSidList = Vec<TypeSid>;
	pub type TypeBidList = Vec<TypeBid>;
	pub type TypeCountList = Vec<TypeCount>;

	pub const ADMIN_TYPE_IS_CREATOR: u8 = 0;
	pub const ADMIN_TYPE_IS_MINTER: u8 = 1;


	#[derive(Encode, Decode, RuntimeDebug, Clone, PartialEq, TypeInfo, Eq, MaxEncodedLen)]
	pub struct StructArtCollectionData<Type> {
		pub name: BoundedVec<Type, TypeCollectionDataLength>,
		pub uri: BoundedVec<Type, TypeCollectionDataLength>,
	}

	#[derive(Encode, Decode, RuntimeDebug, Clone, PartialEq, TypeInfo, Eq, MaxEncodedLen)]
	pub struct StructArtStatus {
		limit: u64,
		art_type: u8,
		locked: bool,
	}

	impl Default for StructArtStatus {
		fn default() -> Self {
			StructArtStatus{
				limit: 0,
				art_type: 0,
				locked: false,
			}
		}
	}


	#[pallet::storage]
	#[pallet::getter(fn administrator_list)]
	#[pallet::unbounded]
	pub type AdministratorList<T: Config> = StorageValue<_, Vec<(T::AccountId, u8)>>;

	#[pallet::storage]
	#[pallet::getter(fn art_collection)]
	pub type ArtCollection<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u64,
		StructArtCollectionData<u8>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn art_satatus)]
	pub type ArtStatus<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u64,
		StructArtStatus,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nft_count)]
	pub type NftCount<T> = StorageMap<
		_,
		Twox64Concat,
		TypeSid, // s_id
		TypeCount, // s_count
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nft_bind_infos)]
	pub type NftBindInfos<T> = StorageMap<
		_,
		Twox64Concat,
		(TypeBid, TypeSid), // b_id
		TypeCount, // s_id
		ValueQuery,
	>;



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },

		ArtCollectionCreated {
			s_id: u64,
			name: BoundedVec<u8, TypeCollectionDataLength>,
			uri: BoundedVec<u8, TypeCollectionDataLength>,
		},

		ArtCollectionUpdated {
			s_id: u64,
			name: BoundedVec<u8, TypeCollectionDataLength>,
			uri: BoundedVec<u8, TypeCollectionDataLength>,
		},

		MintArtOwner {
			bn: <frame_system::Pallet<T>  as BlockNumberProvider>::BlockNumber,
			b_ids: TypeBidList,
			s_ids: TypeSidList,
			count: TypeCountList,
		},

		UpdateAdministratorList {
			administrator_list: Vec<(T::AccountId ,u8)>,
		},

		UpdateArtStatus {
			s_id: u64,
			limit: u64,
			art_type: u8,
			locked: bool,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		//
		NotAdministrator,
		//
		ArtCollectionNotFound,
		//
		ArtCollectionIsLocked,
		//
		ArtCollectionIsExists,
		//
		LengthNotMatch,
		//
		BatchSizeExceeded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create_art_collection(
			origin: OriginFor<T>,
			s_id: TypeSid,
			name: BoundedVec<u8, TypeCollectionDataLength>,
			uri: BoundedVec<u8, TypeCollectionDataLength>
		) -> DispatchResult {

			let who = ensure_signed(origin)?;
			ensure!(Self::is_administrator_list(&who, &ADMIN_TYPE_IS_CREATOR), Error::<T>::NotAdministrator);

			ensure!(!ArtCollection::<T>::contains_key(s_id.clone()), Error::<T>::ArtCollectionIsExists);

			// Update storage.
			<ArtCollection<T>>::insert(s_id.clone(), StructArtCollectionData{
				name: name.clone(),
				uri: uri.clone(),
			});

			// Emit an event.
			Self::deposit_event(Event::ArtCollectionCreated { s_id, name, uri });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn update_art_collection(
			origin: OriginFor<T>,
			s_id: TypeSid,
			name: BoundedVec<u8, TypeCollectionDataLength>,
			uri: BoundedVec<u8, TypeCollectionDataLength>,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;
			ensure!(Self::is_administrator_list(&who, &ADMIN_TYPE_IS_CREATOR), Error::<T>::NotAdministrator);

			// Get art collection data.
			ensure!(ArtCollection::<T>::contains_key(s_id.clone()), Error::<T>::ArtCollectionNotFound);

			// Update storage.
			<ArtCollection<T>>::insert(s_id.clone(), StructArtCollectionData{
				name: name.clone(),
				uri: uri.clone(),
			});

			// Emit an event.
			Self::deposit_event(Event::ArtCollectionUpdated { s_id, name, uri });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		#[transactional]
		pub fn issue_art_ownership(
			origin: OriginFor<T>,
			b_ids: TypeBidList,
			s_ids: TypeSidList,
			count: TypeCountList,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;
			ensure!(Self::is_administrator_list(&who, &ADMIN_TYPE_IS_MINTER), Error::<T>::NotAdministrator);

			ensure!(s_ids.iter().count() > 0 && s_ids.iter().count() < T::BatchMintSize::get().into(), Error::<T>::BatchSizeExceeded);
			ensure!(s_ids.iter().count() == b_ids.iter().count(), Error::<T>::LengthNotMatch);
			ensure!(s_ids.iter().count() == count.iter().count(), Error::<T>::LengthNotMatch);

			// Check s_ids exists
			for s_id in s_ids.iter() {
				ensure!(ArtCollection::<T>::contains_key(s_id.clone()), Error::<T>::ArtCollectionNotFound);
				// Check ArtStatus locked status
				ensure!(ArtStatus::<T>::get(s_id.clone()).locked == false, Error::<T>::ArtCollectionIsLocked);
			}

			for (com_id, c) in b_ids.iter().zip(s_ids.iter()).zip(count.iter()) {
				// Get old count
				let old_count = NftCount::<T>::get(com_id.1.clone());
				let new_count = old_count.saturating_add(c.clone());
				if new_count.saturating_sub(c.clone()) != old_count {
					return Err(Error::<T>::StorageOverflow.into());
				}
				NftCount::<T>::insert(com_id.1.clone(), new_count);

				let personal_old_count = NftBindInfos::<T>::get((com_id.0.clone(), com_id.1.clone()));
				NftBindInfos::<T>::insert((com_id.0.clone(), com_id.1.clone()),  personal_old_count.saturating_add(c.clone()));
			}
			// Get current block number
			let bn = <frame_system::Pallet<T>>::block_number();
			// Emit an event.
			Self::deposit_event(Event::MintArtOwner { bn, s_ids, b_ids, count });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn update_administrator_list(
			origin: OriginFor<T>,
			administrator_list: Vec<(T::AccountId, u8)>
		) -> DispatchResult {
			ensure_root(origin)?;

			// Update storage.
			if administrator_list.iter().count() == 0 {
				AdministratorList::<T>::set(Option::None);
			} else {
				AdministratorList::<T>::set(Option::Some(administrator_list.clone()));
			}

			// Emit an event.
			Self::deposit_event(Event::UpdateAdministratorList { administrator_list });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn set_collection_status(
			origin: OriginFor<T>,
			s_id: TypeSid,
			locked: bool,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;
			ensure!(Self::is_administrator_list(&who, &ADMIN_TYPE_IS_CREATOR), Error::<T>::NotAdministrator);

			let mut old_art_status = ArtStatus::<T>::get(s_id.clone());
			old_art_status.locked = locked;
			ArtStatus::<T>::insert(s_id.clone(), old_art_status.clone());

			// Emit an event.
			Self::deposit_event(Event::UpdateArtStatus {
				s_id: s_id,
				limit: old_art_status.limit,
				art_type: old_art_status.art_type,
				locked: old_art_status.locked,
			});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

}

impl<T: Config> Pallet<T> {
	fn is_administrator_list(who: &T::AccountId, admin_type: &u8) -> bool {
		if let Some(administrator_list) = AdministratorList::<T>::get()
		{
			return administrator_list.contains(&(who.clone(), admin_type.clone()));
		} else {
			return false;
		}
	}
}

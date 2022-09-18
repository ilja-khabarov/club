#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type ClubIdx = u32;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn clubs)]
	pub type Clubs<T: Config> =
		StorageDoubleMap<_, Twox64Concat, u32, Twox64Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Removing non-existent.
		ClubMemberDoesNotExist,
		/// Trying to add an existing member.
		ClubMemberAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_member(
			origin: OriginFor<T>,
			club_idx: ClubIdx,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			if Clubs::<T>::contains_key(club_idx, &account) {
				return Err(Error::<T>::ClubMemberAlreadyExists.into())
			}
			Clubs::<T>::insert(club_idx, account, true);
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_member(
			origin: OriginFor<T>,
			club_idx: ClubIdx,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			if !Clubs::<T>::contains_key(club_idx, &account) {
				return Err(Error::<T>::ClubMemberDoesNotExist.into())
			}

			Clubs::<T>::remove(club_idx, account);
			Ok(())
		}
	}
}

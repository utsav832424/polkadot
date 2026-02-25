// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! # Hospital Registry Pallet
//!
//! A simple pallet to register and manage hospitals.

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use alloc::vec::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config<RuntimeEvent: From<Event<Self>>> {
		/// The maximum length of a name or location string.
		#[pallet::constant]
		type MaxStringLen: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct HospitalInfo<T: Config> {
		pub name: BoundedVec<u8, T::MaxStringLen>,
		pub location: BoundedVec<u8, T::MaxStringLen>,
	}

	#[pallet::storage]
	pub(super) type Hospitals<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, HospitalInfo<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new hospital was registered. [who]
		HospitalRegistered(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Hospital already registered.
		AlreadyRegistered,
		/// Maximum length exceeded.
		TooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0))]
		pub fn register_hospital(
			origin: OriginFor<T>,
			name: Vec<u8>,
			location: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Hospitals::<T>::contains_key(&who), Error::<T>::AlreadyRegistered);

			let bounded_name: BoundedVec<u8, T::MaxStringLen> =
				name.try_into().map_err(|_| Error::<T>::TooLong)?;
			let bounded_location: BoundedVec<u8, T::MaxStringLen> =
				location.try_into().map_err(|_| Error::<T>::TooLong)?;

			let info = HospitalInfo { name: bounded_name, location: bounded_location };
			Hospitals::<T>::insert(&who, info);

			Self::deposit_event(Event::HospitalRegistered(who));
			Ok(())
		}
	}
}

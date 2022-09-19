//! Benchmarking setup for pallet-club

use super::*;

#[allow(unused)]
use crate::Pallet as PalletClub;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_member {
		let acc: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, 1, acc.clone())
	verify {
		assert_eq!(Clubs::<T>::get(1, acc), true);
	}

	impl_benchmark_test_suite!(PalletClub, crate::mock::new_test_ext(), crate::mock::Test);
}

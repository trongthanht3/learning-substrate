//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let price :u32 = 100;
		let caller: T::AccountId = whitelisted_caller();
	}: mint (RawOrigin::Signed(caller), dnas)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}

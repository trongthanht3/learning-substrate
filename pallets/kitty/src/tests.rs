use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn mint_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Kitty::mint(Origin::signed(1), 100));
		// Read pallet storage and assert an expected result.
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(ERC20::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

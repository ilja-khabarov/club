use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};

use sp_runtime::traits::BadOrigin;
pub const ALICE: u64 = 1u64;
pub const BOB: u64 = 2u64;

#[test]
fn check_add_member() {
	new_test_ext().execute_with(|| {
		// Add some members
		assert_ok!(PalletClub::add_member(Origin::root(), 1, ALICE));
		assert_ok!(PalletClub::add_member(Origin::root(), 1, BOB));
		assert_ok!(PalletClub::add_member(Origin::root(), 0, ALICE));
		assert_ok!(PalletClub::add_member(Origin::root(), 3, BOB));

		// Try to add an existing member
		assert_err!(
			PalletClub::add_member(Origin::root(), 1, BOB),
			Error::<Test>::ClubMemberAlreadyExists
		);

		// Try to add with non-root origin
		assert_err!(PalletClub::add_member(Origin::signed(ALICE), 1, BOB), BadOrigin);
		assert_err!(PalletClub::add_member(Origin::signed(BOB), 1, BOB), BadOrigin);
	});
}

#[test]
fn check_add_remove() {
	new_test_ext().execute_with(|| {
		// Add-remove
		assert_ok!(PalletClub::add_member(Origin::root(), 1, ALICE));
		assert_ok!(PalletClub::add_member(Origin::root(), 1, BOB));
		assert_ok!(PalletClub::remove_member(Origin::root(), 1, ALICE));
		assert_ok!(PalletClub::remove_member(Origin::root(), 1, BOB));

		// Add something we already removed to be sure we've really removed it
		assert_ok!(PalletClub::add_member(Origin::root(), 1, ALICE));

		// Try to remove what we already removed
		assert_err!(
			PalletClub::remove_member(Origin::root(), 1, BOB),
			Error::<Test>::ClubMemberDoesNotExist
		);

		// Try to remove with non-root origin
		assert_err!(PalletClub::remove_member(Origin::signed(ALICE), 1, ALICE), BadOrigin);
	});
}

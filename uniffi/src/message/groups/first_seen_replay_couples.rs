use cesrox_core::error::CesrResult;
use cesrox_core::{Matter, Group};
pub use cesrox_core::message::groups::{
    FirstSeenReplayCouple,
    FirstSeenReplayCouples,
};

pub fn first_seen_replay_couple_create(firner: Matter, dater: Matter) -> FirstSeenReplayCouple {
    FirstSeenReplayCouple::new(firner, dater)
}

pub fn first_seen_replay_couples_create(value: Vec<FirstSeenReplayCouple>) -> FirstSeenReplayCouples {
    FirstSeenReplayCouples::new(value)
}

pub fn first_seen_replay_couples_qb64(first_seen_replay_couples: &FirstSeenReplayCouples) -> CesrResult<String> {
    first_seen_replay_couples.qb64()
}

pub fn first_seen_replay_couples_qb64b(first_seen_replay_couples: &FirstSeenReplayCouples) -> CesrResult<Vec<u8>> {
    first_seen_replay_couples.qb64b()
}

pub fn first_seen_replay_couples_qb2(first_seen_replay_couples: &FirstSeenReplayCouples) -> CesrResult<Vec<u8>> {
    first_seen_replay_couples.qb2()
}

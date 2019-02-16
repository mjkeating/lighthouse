use crate::test_utils::TestRandom;
use crate::{Hash256, Slot};
use rand::RngCore;
use serde_derive::Serialize;
use ssz::{hash, Decodable, DecodeError, Encodable, SszStream, TreeHash};

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct ProposalSignedData {
    pub slot: Slot,
    pub shard: u64,
    pub block_root: Hash256,
}

impl Encodable for ProposalSignedData {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append(&self.slot);
        s.append(&self.shard);
        s.append(&self.block_root);
    }
}

impl Decodable for ProposalSignedData {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (slot, i) = <_>::ssz_decode(bytes, i)?;
        let (shard, i) = <_>::ssz_decode(bytes, i)?;
        let (block_root, i) = <_>::ssz_decode(bytes, i)?;

        Ok((
            ProposalSignedData {
                slot,
                shard,
                block_root,
            },
            i,
        ))
    }
}

impl TreeHash for ProposalSignedData {
    fn hash_tree_root_internal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.slot.hash_tree_root_internal());
        result.append(&mut self.shard.hash_tree_root_internal());
        result.append(&mut self.block_root.hash_tree_root_internal());
        hash(&result)
    }
}

impl<T: RngCore> TestRandom<T> for ProposalSignedData {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            slot: <_>::random_for_test(rng),
            shard: <_>::random_for_test(rng),
            block_root: <_>::random_for_test(rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};
    use ssz::ssz_encode;

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = ProposalSignedData::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);
        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    pub fn test_hash_tree_root_internal() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = ProposalSignedData::random_for_test(&mut rng);

        let result = original.hash_tree_root_internal();

        assert_eq!(result.len(), 32);
        // TODO: Add further tests
        // https://github.com/sigp/lighthouse/issues/170
    }
}

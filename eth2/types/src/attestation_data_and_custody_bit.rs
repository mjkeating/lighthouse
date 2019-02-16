use super::AttestationData;
use crate::test_utils::TestRandom;
use rand::RngCore;
use serde_derive::Serialize;
use ssz::{Decodable, DecodeError, Encodable, SszStream, TreeHash};

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct AttestationDataAndCustodyBit {
    pub data: AttestationData,
    pub custody_bit: bool,
}

impl Encodable for AttestationDataAndCustodyBit {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append(&self.data);
        // TODO: deal with bools
    }
}

impl Decodable for AttestationDataAndCustodyBit {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (data, i) = <_>::ssz_decode(bytes, i)?;
        let custody_bit = false;

        let attestation_data_and_custody_bit = AttestationDataAndCustodyBit { data, custody_bit };

        Ok((attestation_data_and_custody_bit, i))
    }
}

impl TreeHash for AttestationDataAndCustodyBit {
    fn hash_tree_root_internal(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.data.hash_tree_root_internal());
        // TODO: add bool ssz
        // result.append(custody_bit.hash_tree_root_internal());
        ssz::hash(&result)
    }
}

impl<T: RngCore> TestRandom<T> for AttestationDataAndCustodyBit {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            data: <_>::random_for_test(rng),
            // TODO: deal with bools
            custody_bit: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};
    use ssz::ssz_encode;

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);

        let original = AttestationDataAndCustodyBit::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);

        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    pub fn test_hash_tree_root_internal() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = AttestationDataAndCustodyBit::random_for_test(&mut rng);

        let result = original.hash_tree_root_internal();

        assert_eq!(result.len(), 32);
        // TODO: Add further tests
        // https://github.com/sigp/lighthouse/issues/170
    }
}

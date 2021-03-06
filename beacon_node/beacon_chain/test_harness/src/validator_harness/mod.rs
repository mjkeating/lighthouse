mod direct_beacon_node;
mod direct_duties;
mod local_signer;

use attester::Attester;
use beacon_chain::BeaconChain;
use block_proposer::PollOutcome as BlockPollOutcome;
use block_proposer::{BlockProducer, Error as BlockPollError};
use db::MemoryDB;
use direct_beacon_node::DirectBeaconNode;
use direct_duties::DirectDuties;
use fork_choice::BitwiseLMDGhost;
use local_signer::LocalSigner;
use slot_clock::TestingSlotClock;
use std::sync::Arc;
use types::{BeaconBlock, ChainSpec, Keypair, Slot};

#[derive(Debug, PartialEq)]
pub enum BlockProduceError {
    DidNotProduce(BlockPollOutcome),
    PollError(BlockPollError),
}

type TestingBlockProducer = BlockProducer<
    TestingSlotClock,
    DirectBeaconNode<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>,
    DirectDuties<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>,
    LocalSigner,
>;

type TestingAttester = Attester<
    TestingSlotClock,
    DirectBeaconNode<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>,
    DirectDuties<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>,
    LocalSigner,
>;

/// A `BlockProducer` and `Attester` which sign using a common keypair.
///
/// The test validator connects directly to a borrowed `BeaconChain` struct. It is useful for
/// testing that the core proposer and attester logic is functioning. Also for supporting beacon
/// chain tests.
pub struct ValidatorHarness {
    pub block_producer: TestingBlockProducer,
    pub attester: TestingAttester,
    pub spec: Arc<ChainSpec>,
    pub epoch_map: Arc<DirectDuties<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>>,
    pub keypair: Keypair,
    pub beacon_node: Arc<DirectBeaconNode<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>>,
    pub slot_clock: Arc<TestingSlotClock>,
    pub signer: Arc<LocalSigner>,
}

impl ValidatorHarness {
    /// Create a new ValidatorHarness that signs with the given keypair, operates per the given spec and connects to the
    /// supplied beacon node.
    ///
    /// A `BlockProducer` and `Attester` is created..
    pub fn new(
        keypair: Keypair,
        beacon_chain: Arc<BeaconChain<MemoryDB, TestingSlotClock, BitwiseLMDGhost<MemoryDB>>>,
        spec: Arc<ChainSpec>,
    ) -> Self {
        let slot_clock = Arc::new(TestingSlotClock::new(spec.genesis_slot.as_u64()));
        let signer = Arc::new(LocalSigner::new(keypair.clone()));
        let beacon_node = Arc::new(DirectBeaconNode::new(beacon_chain.clone()));
        let epoch_map = Arc::new(DirectDuties::new(keypair.pk.clone(), beacon_chain.clone()));

        let block_producer = BlockProducer::new(
            spec.clone(),
            epoch_map.clone(),
            slot_clock.clone(),
            beacon_node.clone(),
            signer.clone(),
        );

        let attester = Attester::new(
            epoch_map.clone(),
            slot_clock.clone(),
            beacon_node.clone(),
            signer.clone(),
        );

        Self {
            block_producer,
            attester,
            spec,
            epoch_map,
            keypair,
            beacon_node,
            slot_clock,
            signer,
        }
    }

    /// Run the `poll` function on the `BlockProducer` and produce a block.
    ///
    /// An error is returned if the producer refuses to produce.
    pub fn produce_block(&mut self) -> Result<BeaconBlock, BlockProduceError> {
        // Using `DirectBeaconNode`, the validator will always return sucessufully if it tries to
        // publish a block.
        match self.block_producer.poll() {
            Ok(BlockPollOutcome::BlockProduced(_)) => {}
            Ok(outcome) => return Err(BlockProduceError::DidNotProduce(outcome)),
            Err(error) => return Err(BlockProduceError::PollError(error)),
        };
        Ok(self
            .beacon_node
            .last_published_block()
            .expect("Unable to obtain produced block."))
    }

    /// Set the validators slot clock to the specified slot.
    ///
    /// The validators slot clock will always read this value until it is set to something else.
    pub fn set_slot(&mut self, slot: Slot) {
        self.slot_clock.set_slot(slot.as_u64())
    }
}

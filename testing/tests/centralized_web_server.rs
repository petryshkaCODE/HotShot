mod common;

use ark_bls12_381::Parameters as Param381;
use async_compatibility_layer::logging::shutdown_logging;
use blake3::Hasher;
use common::*;
use either::Either::Right;
use hotshot::{
    traits::{
        election::{static_committee::StaticCommittee, vrf::VrfImpl},
        implementations::{CentralizedWebCommChannel, MemoryStorage},
    },
    types::Message,
};
use hotshot_testing::TestNodeImpl;
use hotshot_types::{
    data::{ValidatingLeaf, ValidatingProposal},
    message::QuorumVote,
};
// use hotshot_utils::test_util::shutdown_logging;
use jf_primitives::{signatures::BLSSignatureScheme, vrf::blsvrf::BLSVRFScheme};
use tracing::instrument;

/// Centralized web server network test
#[cfg_attr(
    feature = "tokio-executor",
    tokio::test(flavor = "multi_thread", worker_threads = 2)
)]
#[cfg_attr(feature = "async-std-executor", async_std::test)]
#[instrument]
async fn centralized_server_network() {
    let description = GeneralTestDescriptionBuilder {
        round_start_delay: 25,
        num_bootstrap_nodes: 5,
        timeout_ratio: (11, 10),
        total_nodes: 5,
        start_nodes: 5,
        num_succeeds: 5,
        txn_ids: Right(1),
        next_view_timeout: 3000,
        start_delay: 120000,
        ..GeneralTestDescriptionBuilder::default()
    };

    description
        .build::<StaticCommitteeTestTypes, TestNodeImpl<
            StaticCommitteeTestTypes,
            ValidatingLeaf<StaticCommitteeTestTypes>,
            ValidatingProposal<
                StaticCommitteeTestTypes,
                StaticCommittee<StaticCommitteeTestTypes, ValidatingLeaf<StaticCommitteeTestTypes>>,
            >,
            QuorumVote<StaticCommitteeTestTypes, ValidatingLeaf<StaticCommitteeTestTypes>>,
            CentralizedWebCommChannel<
                StaticCommitteeTestTypes,
                ValidatingProposal<
                    StaticCommitteeTestTypes,
                    StaticCommittee<
                        StaticCommitteeTestTypes,
                        ValidatingLeaf<StaticCommitteeTestTypes>,
                    >,
                >,
                QuorumVote<StaticCommitteeTestTypes, ValidatingLeaf<StaticCommitteeTestTypes>>,
                StaticCommittee<StaticCommitteeTestTypes, ValidatingLeaf<StaticCommitteeTestTypes>>,

            >,
            MemoryStorage<StaticCommitteeTestTypes, ValidatingLeaf<StaticCommitteeTestTypes>>,
            StaticCommittee<StaticCommitteeTestTypes, ValidatingLeaf<StaticCommitteeTestTypes>>,
        >>()
        .execute()
        .await
        .unwrap();
    shutdown_logging();
}

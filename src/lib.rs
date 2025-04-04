
mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use pb::sf::substreams::sink::pubsub::v1 as pubsub;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

// Add these lines
use substreams::errors::Error; 
use std::str::FromStr;
use substreams::scalar::BigDecimal;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

const ATSEAHUB_TRACKED_CONTRACT: [u8; 20] = hex!("20206658b072226e7d1bbd9c611d213358a970a6");


fn map_atseahub_events(blk: &eth::Block, events: &mut contract::Events) {
    events.atseahub_anchor_droppeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::AnchorDropped::match_and_decode(log) {
                        return Some(contract::AtseahubAnchorDropped {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            duration_hours: event.duration_hours.to_string(),
                            geohash: event.geohash,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_anchor_dropped_failures.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::AnchorDroppedFailure::match_and_decode(log) {
                        return Some(contract::AtseahubAnchorDroppedFailure {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            geohash: event.geohash,
                            reason: event.reason,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_at_sea_upgrade_versions.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::AtSeaUpgradeVersion::match_and_decode(log) {
                        return Some(contract::AtseahubAtSeaUpgradeVersion {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version,
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::AtseahubInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_mint_sailor_failures.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::MintSailorFailure::match_and_decode(log) {
                        return Some(contract::AtseahubMintSailorFailure {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            name: event.name,
                            reason: event.reason,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::AtseahubOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_sailor_minteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::SailorMinted::match_and_decode(log) {
                        return Some(contract::AtseahubSailorMinted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            name: event.name,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.atseahub_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ATSEAHUB_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::atseahub_contract::events::Upgraded::match_and_decode(log) {
                        return Some(contract::AtseahubUpgraded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_atseahub_events(&blk, &mut events);
    Ok(events)
}


#[substreams::handlers::map]
fn map_pubsub(events: contract::Events) -> Result<pubsub::Publish, Error> {
    let mut publish = pubsub::Publish::default();

    for anchor in events.atseahub_anchor_droppeds {
        // Build a JSON-like string by hand
        let anchor_json = format!(
            r#"{{"evt_tx_hash":"{}","evt_index":{},"evt_block_number":{},"duration_hours":"{}","geohash":"{}","to":"0x{:x}","token_id":"{}"}}"#,
            anchor.evt_tx_hash,
            anchor.evt_index,
            anchor.evt_block_number,
            anchor.duration_hours,
            anchor.geohash,
            Hex(&anchor.to),
            anchor.token_id,
        );

        let mut msg = pubsub::Message::default();
        msg.data = anchor_json.into_bytes();

        // (Optional) Add attributes
        msg.attributes.push(pubsub::Attribute {
            key: "block_number".to_string(),
            value: anchor.evt_block_number.to_string(),
        });
        msg.attributes.push(pubsub::Attribute {
            key: "tx_hash".to_string(),
            value: anchor.evt_tx_hash,
        });

        publish.messages.push(msg);
    }

    Ok(publish)
}



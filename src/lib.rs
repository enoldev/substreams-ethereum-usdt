mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

const TRACKED_CONTRACT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        added_black_lists: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::AddedBlackList::match_and_decode(log) {
                            return Some(contract::AddedBlackList {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                u_user: event.u_user,
                            });
                        }

                        None
                })
            })
            .collect(),
        approvals: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Approval::match_and_decode(log) {
                            return Some(contract::Approval {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                owner: event.owner,
                                spender: event.spender,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        deprecates: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Deprecate::match_and_decode(log) {
                            return Some(contract::Deprecate {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_address: event.new_address,
                            });
                        }

                        None
                })
            })
            .collect(),
        destroyed_black_funds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::DestroyedBlackFunds::match_and_decode(log) {
                            return Some(contract::DestroyedBlackFunds {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                u_balance: event.u_balance.to_string(),
                                u_black_listed_user: event.u_black_listed_user,
                            });
                        }

                        None
                })
            })
            .collect(),
        issues: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Issue::match_and_decode(log) {
                            return Some(contract::Issue {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        params: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Params::match_and_decode(log) {
                            return Some(contract::Params {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                fee_basis_points: event.fee_basis_points.to_string(),
                                max_fee: event.max_fee.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        pauses: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Pause::match_and_decode(log) {
                            return Some(contract::Pause {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
        redeems: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Redeem::match_and_decode(log) {
                            return Some(contract::Redeem {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        removed_black_lists: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::RemovedBlackList::match_and_decode(log) {
                            return Some(contract::RemovedBlackList {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                u_user: event.u_user,
                            });
                        }

                        None
                })
            })
            .collect(),
        transfers: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Transfer::match_and_decode(log) {
                            return Some(contract::Transfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                value: event.value.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        unpauses: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Unpause::match_and_decode(log) {
                            return Some(contract::Unpause {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    // Loop over all the abis events to create changes
    events.added_black_lists.into_iter().for_each(|evt| {
        tables
            .create_row("added_black_list", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_user", Hex(&evt.u_user).to_string());
    });
    events.approvals.into_iter().for_each(|evt| {
        tables
            .create_row("approval", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.deprecates.into_iter().for_each(|evt| {
        tables
            .create_row("deprecate", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_address", Hex(&evt.new_address).to_string());
    });
    events.destroyed_black_funds.into_iter().for_each(|evt| {
        tables
            .create_row("destroyed_black_funds", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_balance", BigDecimal::from_str(&evt.u_balance).unwrap())
            .set("u_black_listed_user", Hex(&evt.u_black_listed_user).to_string());
    });
    events.issues.into_iter().for_each(|evt| {
        tables
            .create_row("issue", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.params.into_iter().for_each(|evt| {
        tables
            .create_row("params", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("fee_basis_points", BigDecimal::from_str(&evt.fee_basis_points).unwrap())
            .set("max_fee", BigDecimal::from_str(&evt.max_fee).unwrap());
    });
    events.pauses.into_iter().for_each(|evt| {
        tables
            .create_row("pause", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.redeems.into_iter().for_each(|evt| {
        tables
            .create_row("redeem", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.removed_black_lists.into_iter().for_each(|evt| {
        tables
            .create_row("removed_black_list", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_user", Hex(&evt.u_user).to_string());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.unpauses.into_iter().for_each(|evt| {
        tables
            .create_row("unpause", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.added_black_lists.into_iter().for_each(|evt| {
        tables
            .create_row("added_black_list", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_user", Hex(&evt.u_user).to_string());
    });
    events.approvals.into_iter().for_each(|evt| {
        tables
            .create_row("approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("spender", Hex(&evt.spender).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.deprecates.into_iter().for_each(|evt| {
        tables
            .create_row("deprecate", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_address", Hex(&evt.new_address).to_string());
    });
    events.destroyed_black_funds.into_iter().for_each(|evt| {
        tables
            .create_row("destroyed_black_funds", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_balance", BigDecimal::from_str(&evt.u_balance).unwrap())
            .set("u_black_listed_user", Hex(&evt.u_black_listed_user).to_string());
    });
    events.issues.into_iter().for_each(|evt| {
        tables
            .create_row("issue", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.params.into_iter().for_each(|evt| {
        tables
            .create_row("params", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("fee_basis_points", BigDecimal::from_str(&evt.fee_basis_points).unwrap())
            .set("max_fee", BigDecimal::from_str(&evt.max_fee).unwrap());
    });
    events.pauses.into_iter().for_each(|evt| {
        tables
            .create_row("pause", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });
    events.redeems.into_iter().for_each(|evt| {
        tables
            .create_row("redeem", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap());
    });
    events.removed_black_lists.into_iter().for_each(|evt| {
        tables
            .create_row("removed_black_list", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_user", Hex(&evt.u_user).to_string());
    });
    events.transfers.into_iter().for_each(|evt| {
        tables
            .create_row("transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
    events.unpauses.into_iter().for_each(|evt| {
        tables
            .create_row("unpause", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number);
    });

    Ok(tables.to_entity_changes())
}

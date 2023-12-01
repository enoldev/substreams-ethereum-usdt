CREATE TABLE IF NOT EXISTS added_black_list (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "u_user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS approval (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS deprecate (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_address" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS destroyed_black_funds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "u_balance" DECIMAL,
    "u_black_listed_user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS issue (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS params (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "fee_basis_points" DECIMAL,
    "max_fee" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS pause (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS redeem (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS removed_black_list (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "u_user" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS transfer (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS unpause (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);


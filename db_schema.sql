DROP TABLE IF EXISTS WeaveVMArchiver;
DROP TABLE IF EXISTS WeaveVMArchiverBackfill;

CREATE TABLE IF NOT EXISTS WeaveVMArchiver (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE TABLE IF NOT EXISTS WeaveVMArchiverBackfill (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE INDEX idx_archiver_txid ON WeaveVMArchiver (WeaveVMArchiveTxid);
CREATE INDEX idx_backfill_txid ON WeaveVMArchiverBackfill (WeaveVMArchiveTxid);
CREATE INDEX idx_archiver_block_id ON WeaveVMArchiver (NetworkBlockId);
CREATE INDEX idx_backfill_block_id ON WeaveVMArchiverBackfill (NetworkBlockId);

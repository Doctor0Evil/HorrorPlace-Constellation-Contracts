PRAGMA foreign_keys = ON;

----------------------------------------------------------------------
-- Policy change evaluation
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS policy_change_eval (
  eval_id            INTEGER PRIMARY KEY,
  tenant_id          TEXT    NOT NULL,
  policy_name        TEXT    NOT NULL,
  from_snapshot_id   INTEGER NOT NULL,
  to_snapshot_id     INTEGER NOT NULL,
  roh                REAL    NOT NULL,
  veco               REAL    NOT NULL,
  new_egress_count   INTEGER NOT NULL,
  corridor_id        TEXT    NOT NULL,
  within_corridor    INTEGER NOT NULL DEFAULT 0,
  violations_json    TEXT,
  created_at         TEXT    NOT NULL,
  -- Optional linkage to an intent or proposal
  change_id          TEXT,
  intent_id          TEXT,
  -- Optional scalar NEF and budget for richer governance
  nef                REAL,
  impact_budget      REAL,
  CONSTRAINT chk_eval_bounds CHECK (
    roh  >= 0.0 AND roh  <= 1.0 AND
    veco >= 0.0 AND veco <= 1.0 AND
    new_egress_count >= 0
  )
);

CREATE INDEX IF NOT EXISTS idx_policy_change_eval_tenant
  ON policy_change_eval (tenant_id, policy_name, created_at);

CREATE INDEX IF NOT EXISTS idx_policy_change_eval_corridor
  ON policy_change_eval (tenant_id, corridor_id, created_at);

CREATE INDEX IF NOT EXISTS idx_policy_change_eval_snapshots
  ON policy_change_eval (tenant_id, from_snapshot_id, to_snapshot_id);

CREATE INDEX IF NOT EXISTS idx_policy_change_eval_within
  ON policy_change_eval (tenant_id, within_corridor);

----------------------------------------------------------------------
-- iptables rules snapshot
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS iptables_rules (
  rule_id          INTEGER PRIMARY KEY,
  tenant_id        TEXT    NOT NULL,
  snapshot_id      INTEGER NOT NULL,
  table_name       TEXT    NOT NULL,
  chain_name       TEXT    NOT NULL,
  rule_index       INTEGER NOT NULL,
  raw_line         TEXT    NOT NULL,

  -- Basic selectors
  src_cidr         TEXT,
  dst_cidr         TEXT,
  in_iface         TEXT,
  out_iface        TEXT,
  protocol         TEXT,

  -- Core target / jump information
  target           TEXT    NOT NULL,
  target_chain     TEXT,
  goto_chain       TEXT,

  -- TCP/UDP ports
  src_port_min     INTEGER,
  src_port_max     INTEGER,
  dst_port_min     INTEGER,
  dst_port_max     INTEGER,

  -- Conntrack
  ct_state         TEXT,

  -- Owner match
  owner_uid_min    INTEGER,
  owner_uid_max    INTEGER,
  owner_gid_min    INTEGER,
  owner_gid_max    INTEGER,
  owner_pid        INTEGER,
  owner_sid        INTEGER,

  -- ipset
  ipset_name       TEXT,
  ipset_dir        TEXT,

  -- Misc match modules and extensions
  matches_json     TEXT,
  comments_json    TEXT,

  -- Governance linkage
  logical_rule_id  TEXT,
  created_at       TEXT    NOT NULL,

  CONSTRAINT uq_iptables_rules UNIQUE (
    tenant_id,
    snapshot_id,
    table_name,
    chain_name,
    rule_index
  ),

  FOREIGN KEY (snapshot_id) REFERENCES graph_snapshot (snapshot_id)
);

CREATE INDEX IF NOT EXISTS idx_iptables_rules_chain
  ON iptables_rules (tenant_id, snapshot_id, table_name, chain_name, rule_index);

CREATE INDEX IF NOT EXISTS idx_iptables_rules_target
  ON iptables_rules (tenant_id, snapshot_id, target);

CREATE INDEX IF NOT EXISTS idx_iptables_rules_cidr
  ON iptables_rules (tenant_id, snapshot_id, src_cidr, dst_cidr);

CREATE INDEX IF NOT EXISTS idx_iptables_rules_logical
  ON iptables_rules (tenant_id, snapshot_id, logical_rule_id);

CREATE INDEX IF NOT EXISTS idx_iptables_rules_ipset
  ON iptables_rules (tenant_id, snapshot_id, ipset_name, ipset_dir);

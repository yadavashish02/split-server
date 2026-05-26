-- =========================================================
-- USERS
-- =========================================================
CREATE TABLE users (
    id BLOB PRIMARY KEY,
    username TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX idx_users_username ON users(username);

-- =========================================================
-- CURRENCIES
-- =========================================================
CREATE TABLE currencies (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    minor_unit INTEGER NOT NULL
);

INSERT INTO currencies VALUES
('INR','Indian Rupee','₹',2),
('USD','US Dollar','$',2),
('EUR','Euro','€',2),
('JPY','Japanese Yen','¥',0);

-- =========================================================
-- GROUPS + MEMBERSHIP
-- =========================================================
CREATE TABLE groups (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    created_by BLOB NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE TABLE group_members (
    group_id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',
    joined_at INTEGER NOT NULL DEFAULT (unixepoch()),
    PRIMARY KEY (group_id, user_id),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_group_members_user ON group_members(user_id);

-- =========================================================
-- CATEGORIES
-- =========================================================
CREATE TABLE categories (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    icon TEXT,
    parent_id BLOB,
    created_by BLOB,
    group_id BLOB,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (parent_id) REFERENCES categories(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (group_id) REFERENCES groups(id)
);

INSERT INTO categories(id,name,icon) VALUES
('food','Food','🍔'),
('travel','Travel','✈️'),
('rent','Rent','🏠'),
('shopping','Shopping','🛍️'),
('utilities','Utilities','💡'),
('entertainment','Entertainment','🎬');

-- =========================================================
-- EXPENSES
-- =========================================================
CREATE TABLE expenses (
    id BLOB PRIMARY KEY,
    group_id BLOB NOT NULL,
    paid_by BLOB NOT NULL,
    description TEXT,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    split_type TEXT NOT NULL,
    category_id BLOB,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (paid_by) REFERENCES users(id),
    FOREIGN KEY (currency) REFERENCES currencies(code),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE INDEX idx_expenses_group ON expenses(group_id);

-- =========================================================
-- EXPENSE SPLITS
-- =========================================================
CREATE TABLE expense_splits (
    expense_id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    amount_owed INTEGER NOT NULL,
    PRIMARY KEY (expense_id, user_id),
    FOREIGN KEY (expense_id) REFERENCES expenses(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_splits_user ON expense_splits(user_id);

-- =========================================================
-- MULTI-CURRENCY LEDGER (PAIRWISE)
-- =========================================================
CREATE TABLE user_balances (
    group_id BLOB NOT NULL,
    from_user BLOB NOT NULL,
    to_user BLOB NOT NULL,
    currency TEXT NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY (group_id, from_user, to_user, currency),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (from_user) REFERENCES users(id),
    FOREIGN KEY (to_user) REFERENCES users(id),
    FOREIGN KEY (currency) REFERENCES currencies(code),
    CHECK (from_user != to_user)
);

CREATE INDEX idx_balances_group ON user_balances(group_id);
CREATE INDEX idx_balances_user_from ON user_balances(from_user);
CREATE INDEX idx_balances_user_to ON user_balances(to_user);

-- =========================================================
-- MATERIALIZED NET BALANCES (GRAPH BYPASS)
-- =========================================================
CREATE TABLE group_member_net_balances (
    group_id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    currency TEXT NOT NULL,
    net_amount INTEGER NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    PRIMARY KEY (group_id, user_id, currency),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (currency) REFERENCES currencies(code)
);

CREATE INDEX idx_net_balances_group ON group_member_net_balances(group_id);
CREATE INDEX idx_net_balances_user ON group_member_net_balances(user_id);

-- =========================================================
-- PAYMENTS (SETTLEMENTS)
-- =========================================================
CREATE TABLE payments (
    id BLOB PRIMARY KEY,
    group_id BLOB NOT NULL,
    paid_by BLOB NOT NULL,
    paid_to BLOB NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    note TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (paid_by) REFERENCES users(id),
    FOREIGN KEY (paid_to) REFERENCES users(id),
    FOREIGN KEY (currency) REFERENCES currencies(code),
    CHECK (paid_by != paid_to)
);

-- =========================================================
-- INVITES
-- =========================================================
CREATE TABLE invites (
    id BLOB PRIMARY KEY,
    group_id BLOB NOT NULL,
    invited_by BLOB NOT NULL,
    email TEXT,
    invited_user_id BLOB,
    status TEXT NOT NULL DEFAULT 'pending',
    token TEXT NOT NULL UNIQUE,
    expires_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);

-- =========================================================
-- ACTIVITY LOG
-- =========================================================
CREATE TABLE activity_log (
    id BLOB PRIMARY KEY,
    actor_user_id BLOB NOT NULL,
    group_id BLOB,
    entity_type TEXT NOT NULL,
    entity_id BLOB,
    action TEXT NOT NULL,
    metadata_json TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX idx_activity_group_time
ON activity_log(group_id, created_at DESC);

-- =========================================================
-- TRIGGERS → UPDATE NET BALANCES FROM LEDGER
-- =========================================================
CREATE TRIGGER ledger_insert_update_net
AFTER INSERT ON user_balances
BEGIN
    INSERT INTO group_member_net_balances
        (group_id,user_id,currency,net_amount)
    VALUES (NEW.group_id,NEW.from_user,NEW.currency,-NEW.amount)
    ON CONFLICT(group_id,user_id,currency)
    DO UPDATE SET net_amount = net_amount - NEW.amount;

    INSERT INTO group_member_net_balances
        (group_id,user_id,currency,net_amount)
    VALUES (NEW.group_id,NEW.to_user,NEW.currency,NEW.amount)
    ON CONFLICT(group_id,user_id,currency)
    DO UPDATE SET net_amount = net_amount + NEW.amount;
END;

CREATE TRIGGER cleanup_zero_net
AFTER UPDATE ON group_member_net_balances
WHEN NEW.net_amount = 0
BEGIN
    DELETE FROM group_member_net_balances
    WHERE group_id = NEW.group_id
      AND user_id = NEW.user_id
      AND currency = NEW.currency;
END;

-- =========================================================
-- SIMPLE RLS USING CURRENT USER TEMP TABLE
-- =========================================================
CREATE TEMP TABLE current_user(id BLOB);

CREATE VIEW v_groups AS
SELECT g.*
FROM groups g
JOIN group_members gm ON gm.group_id=g.id
JOIN current_user cu ON cu.id=gm.user_id;

CREATE VIEW v_expenses AS
SELECT e.*
FROM expenses e
JOIN group_members gm ON gm.group_id=e.group_id
JOIN current_user cu ON cu.id=gm.user_id;

CREATE VIEW v_net_balances AS
SELECT *
FROM group_member_net_balances nb
JOIN current_user cu
WHERE nb.user_id = cu.id;

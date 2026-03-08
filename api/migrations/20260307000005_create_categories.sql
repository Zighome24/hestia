CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    color VARCHAR(7) NOT NULL DEFAULT '#6b7280',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE receipt_categories (
    receipt_id UUID NOT NULL REFERENCES receipts(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (receipt_id, category_id)
);
CREATE INDEX idx_receipt_categories_category_id ON receipt_categories(category_id);

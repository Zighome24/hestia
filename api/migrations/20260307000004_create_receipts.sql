CREATE TABLE receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    card_id UUID REFERENCES cards(id) ON DELETE SET NULL,
    total_amount BIGINT NOT NULL,
    photo_path VARCHAR(512),
    notes TEXT,
    purchased_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_receipts_user_id ON receipts(user_id);
CREATE INDEX idx_receipts_purchased_at ON receipts(purchased_at);

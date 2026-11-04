CREATE TABLE spots (
    id UUID PRIMARY KEY,

    name VARCHAR(100) NOT NULL,
    symbol VARCHAR(30) NOT NULL UNIQUE,
    image_url TEXT NOT NULL,

    base_symbol VARCHAR(30) NOT NULL,
    quote_symbol VARCHAR(30) NOT NULL,

    is_visible BOOLEAN NOT NULL DEFAULT TRUE,

    status VARCHAR(20) NOT NULL DEFAULT 'active',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT spots_status_check
      CHECK (status IN ('active', 'inactive', 'paused', 'suspended')),

    CONSTRAINT spots_base_quote_check
      CHECK (base_symbol <> quote_symbol),
    
    CONSTRAINT spots_base_quote_unique
      UNIQUE (base_symbol, quote_symbol)
);

CREATE INDEX idx_spots_visibility_status
    ON spots (is_visible, status);

CREATE INDEX idx_spots_created_at
    ON spots (created_at DESC);
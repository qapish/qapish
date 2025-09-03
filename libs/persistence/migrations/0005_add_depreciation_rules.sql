-- Migration: Add depreciation rules for package pricing
-- This table stores configurable depreciation rules for each package
-- Used to calculate reduced pricing for equipment with usage hours

CREATE TABLE IF NOT EXISTS package_depreciation_rules (
    id SERIAL PRIMARY KEY,
    package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    -- The percentage of original price after full depreciation (default 25%)
    final_depreciated_percentage DECIMAL(5,2) NOT NULL DEFAULT 25.00 CHECK (final_depreciated_percentage >= 0 AND final_depreciated_percentage <= 100),
    -- The number of hours for full depreciation (default 26280 hours = 3 years * 365 days * 24 hours)
    full_depreciation_hours INTEGER NOT NULL DEFAULT 26280 CHECK (full_depreciation_hours > 0),
    -- Optional: depreciation curve type (linear, exponential, etc.)
    depreciation_curve VARCHAR(20) NOT NULL DEFAULT 'linear' CHECK (depreciation_curve IN ('linear', 'exponential', 'stepped')),
    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(package_id)
);

-- Create index for faster lookups
CREATE INDEX idx_package_depreciation_rules_package_id ON package_depreciation_rules(package_id);

-- Insert default depreciation rules for existing packages
INSERT INTO package_depreciation_rules (package_id, final_depreciated_percentage, full_depreciation_hours, depreciation_curve)
SELECT
    id,
    25.00, -- 25% of original price after full depreciation
    26280, -- 3 years in hours
    'linear'
FROM packages
ON CONFLICT (package_id) DO NOTHING;

-- Add trigger to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_package_depreciation_rules_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_package_depreciation_rules_updated_at
    BEFORE UPDATE ON package_depreciation_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_package_depreciation_rules_updated_at();

-- Add trigger to automatically create depreciation rules for new packages
CREATE OR REPLACE FUNCTION create_default_depreciation_rule()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO package_depreciation_rules (package_id, final_depreciated_percentage, full_depreciation_hours, depreciation_curve)
    VALUES (NEW.id, 25.00, 26280, 'linear')
    ON CONFLICT (package_id) DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_create_default_depreciation_rule
    AFTER INSERT ON packages
    FOR EACH ROW
    EXECUTE FUNCTION create_default_depreciation_rule();

-- Add comments for documentation
COMMENT ON TABLE package_depreciation_rules IS 'Stores depreciation rules for calculating adjusted pricing on used equipment';
COMMENT ON COLUMN package_depreciation_rules.final_depreciated_percentage IS 'Percentage of original price remaining after full depreciation (e.g., 25 means 25% of original price)';
COMMENT ON COLUMN package_depreciation_rules.full_depreciation_hours IS 'Number of usage hours for equipment to reach full depreciation';
COMMENT ON COLUMN package_depreciation_rules.depreciation_curve IS 'Type of depreciation curve: linear (straight-line), exponential (faster initial depreciation), or stepped (discrete steps)';

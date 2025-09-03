-- Migration: Refactor provenance into separate table
-- This allows multiple provenance states (new/used with different hours) per package

-- Create package_provenance table
CREATE TABLE IF NOT EXISTS package_provenance (
    id SERIAL PRIMARY KEY,
    package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    provenance_type VARCHAR(20) NOT NULL CHECK (provenance_type IN ('new', 'used')),
    usage_hours INTEGER DEFAULT 0 CHECK (usage_hours >= 0),
    quantity_available INTEGER NOT NULL DEFAULT 0 CHECK (quantity_available >= 0),
    is_active BOOLEAN NOT NULL DEFAULT true,
    -- Pricing can be calculated based on usage_hours and depreciation rules
    -- But we'll cache it here for performance
    calculated_price_usdc INTEGER,
    discount_percentage DECIMAL(5,2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    -- Ensure unique combination of package and usage hours
    UNIQUE(package_id, usage_hours)
);

-- Create indexes for performance
CREATE INDEX idx_package_provenance_package_id ON package_provenance(package_id);
CREATE INDEX idx_package_provenance_active ON package_provenance(is_active);
CREATE INDEX idx_package_provenance_type ON package_provenance(provenance_type);

-- Migrate existing provenance data from packages table
INSERT INTO package_provenance (
    package_id,
    provenance_type,
    usage_hours,
    quantity_available,
    is_active
)
SELECT
    id as package_id,
    provenance_type,
    COALESCE(provenance_value, 0) as usage_hours,
    CASE
        WHEN availability_type = 'in_stock' THEN 10
        WHEN availability_type = 'preorder' THEN 5
        ELSE 1
    END as quantity_available,
    true as is_active
FROM packages
WHERE is_active = true;

-- Add some example used equipment variants for existing packages
-- This gives each package multiple provenance options
INSERT INTO package_provenance (package_id, provenance_type, usage_hours, quantity_available, is_active)
SELECT
    id,
    'used',
    8760,  -- 1 year of usage (365 * 24)
    3,
    true
FROM packages
WHERE is_active = true
ON CONFLICT (package_id, usage_hours) DO NOTHING;

INSERT INTO package_provenance (package_id, provenance_type, usage_hours, quantity_available, is_active)
SELECT
    id,
    'used',
    17520,  -- 2 years of usage
    2,
    true
FROM packages
WHERE is_active = true
ON CONFLICT (package_id, usage_hours) DO NOTHING;

-- Function to calculate depreciated price
CREATE OR REPLACE FUNCTION calculate_depreciated_price(
    p_package_id UUID,
    p_usage_hours INTEGER
) RETURNS TABLE (
    calculated_price INTEGER,
    discount_pct DECIMAL(5,2)
) AS $$
DECLARE
    v_original_price INTEGER;
    v_final_pct DECIMAL(5,2);
    v_full_hours INTEGER;
    v_depreciation_curve VARCHAR(20);
    v_calculated_price INTEGER;
    v_discount_pct DECIMAL(5,2);
BEGIN
    -- Get original price from package
    SELECT setup_price_usdc INTO v_original_price
    FROM packages
    WHERE id = p_package_id;

    -- Get depreciation rules
    SELECT
        final_depreciated_percentage,
        full_depreciation_hours,
        depreciation_curve
    INTO v_final_pct, v_full_hours, v_depreciation_curve
    FROM package_depreciation_rules
    WHERE package_id = p_package_id;

    -- If no depreciation rule exists, use defaults
    IF v_final_pct IS NULL THEN
        v_final_pct := 25.00;
        v_full_hours := 26280; -- 3 years
        v_depreciation_curve := 'linear';
    END IF;

    -- Calculate price based on usage
    IF p_usage_hours = 0 THEN
        v_calculated_price := v_original_price;
        v_discount_pct := 0;
    ELSIF p_usage_hours >= v_full_hours THEN
        v_calculated_price := (v_original_price * v_final_pct / 100)::INTEGER;
        v_discount_pct := 100 - v_final_pct;
    ELSE
        -- Linear depreciation calculation
        DECLARE
            v_usage_ratio DECIMAL(10,6);
            v_price_range DECIMAL(10,2);
            v_remaining_pct DECIMAL(10,2);
        BEGIN
            v_usage_ratio := p_usage_hours::DECIMAL / v_full_hours;
            v_price_range := 100 - v_final_pct;
            v_remaining_pct := 100 - (v_price_range * v_usage_ratio);
            v_calculated_price := (v_original_price * v_remaining_pct / 100)::INTEGER;
            v_discount_pct := 100 - v_remaining_pct;
        END;
    END IF;

    RETURN QUERY SELECT v_calculated_price, v_discount_pct;
END;
$$ LANGUAGE plpgsql;

-- Update calculated prices for all provenance records
UPDATE package_provenance pp
SET
    calculated_price_usdc = calc.calculated_price,
    discount_percentage = calc.discount_pct
FROM (
    SELECT
        pp2.id,
        (calculate_depreciated_price(pp2.package_id, pp2.usage_hours)).*
    FROM package_provenance pp2
) calc
WHERE pp.id = calc.id;

-- Add trigger to auto-update calculated prices when provenance is inserted or updated
CREATE OR REPLACE FUNCTION update_provenance_calculated_price()
RETURNS TRIGGER AS $$
BEGIN
    SELECT calculated_price, discount_pct
    INTO NEW.calculated_price_usdc, NEW.discount_percentage
    FROM calculate_depreciated_price(NEW.package_id, NEW.usage_hours);

    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_provenance_price
    BEFORE INSERT OR UPDATE ON package_provenance
    FOR EACH ROW
    EXECUTE FUNCTION update_provenance_calculated_price();

-- Create view for easy querying of packages with their provenance options
CREATE OR REPLACE VIEW package_with_provenance AS
SELECT
    p.*,
    pp.id as provenance_id,
    pp.provenance_type,
    pp.usage_hours,
    pp.quantity_available,
    pp.calculated_price_usdc,
    pp.discount_percentage,
    pp.is_active as provenance_active
FROM packages p
LEFT JOIN package_provenance pp ON p.id = pp.package_id
WHERE p.is_active = true AND pp.is_active = true
ORDER BY p.setup_price_usdc, pp.usage_hours;

-- Now we can safely drop the old provenance columns from packages table
ALTER TABLE packages DROP COLUMN IF EXISTS provenance_type;
ALTER TABLE packages DROP COLUMN IF EXISTS provenance_value;

-- Add comments for documentation
COMMENT ON TABLE package_provenance IS 'Stores different provenance states (new/used) for each package';
COMMENT ON COLUMN package_provenance.usage_hours IS 'Number of hours the equipment has been used (0 for new)';
COMMENT ON COLUMN package_provenance.quantity_available IS 'Number of units available at this provenance level';
COMMENT ON COLUMN package_provenance.calculated_price_usdc IS 'Cached calculated price after depreciation';
COMMENT ON COLUMN package_provenance.discount_percentage IS 'Percentage discount from original price';

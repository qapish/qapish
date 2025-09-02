-- Update GMKtek package to use Radeon_8060S with proper memory configuration
-- 128GB total: 32GB system RAM + 96GB GPU RAM
UPDATE packages
SET
    gpu_class = 'Radeon_8060S',
    gpu_count = 1,
    ram_gb = 32,
    vram_gb = 96,
    hardware_description = 'GMKtek X2 (or similar) with AMD Radeon 8060S 96GB and 32GB system RAM'
WHERE id = '550e8400-e29b-41d4-a716-446655440001';

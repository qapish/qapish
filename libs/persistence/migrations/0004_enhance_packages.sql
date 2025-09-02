-- Add new fields to packages table
alter table packages add column sku text;
alter table packages add column availability_type text not null default 'in_stock';
alter table packages add column availability_value integer;
alter table packages add column provenance_type text not null default 'new';
alter table packages add column provenance_value integer;

-- Create package images table
create table package_images (
  id uuid primary key default gen_random_uuid(),
  package_id uuid not null references packages(id) on delete cascade,
  filename text not null,
  title text not null,
  description text not null,
  sort_order integer not null default 0,
  created_at timestamptz not null default now()
);

-- Update existing packages with SKUs
update packages set sku = '1x-a8060-96' where id = '550e8400-e29b-41d4-a716-446655440001';
update packages set sku = '2x-n5090-64' where id = '550e8400-e29b-41d4-a716-446655440002';
update packages set sku = '2x-h100-160' where id = '550e8400-e29b-41d4-a716-446655440003';

-- Set availability for existing packages
update packages set availability_type = 'in_stock' where id = '550e8400-e29b-41d4-a716-446655440001';
update packages set availability_type = 'build', availability_value = 48 where id = '550e8400-e29b-41d4-a716-446655440002';
update packages set availability_type = 'preorder' where id = '550e8400-e29b-41d4-a716-446655440003';

-- Set provenance for existing packages
update packages set provenance_type = 'new' where id in ('550e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002');
update packages set provenance_type = 'used', provenance_value = 1500 where id = '550e8400-e29b-41d4-a716-446655440003';

-- Add package images with actual SVG assets
insert into package_images (package_id, filename, title, description, sort_order) values
-- Midrange Consumer
('550e8400-e29b-41d4-a716-446655440001', '/packages/1x-a8060-96-hero.svg', 'AMD A8060 Server', 'Professional AI compute server with 96GB HBM3e memory', 1),
('550e8400-e29b-41d4-a716-446655440001', '/packages/1x-a8060-96-specs.svg', 'Technical Specifications', 'Detailed hardware specifications and performance metrics', 2),

-- Top Consumer
('550e8400-e29b-41d4-a716-446655440002', '/packages/2x-n5090-64-hero.svg', 'Dual N5090 Configuration', 'High-performance dual GPU setup with 64GB total VRAM', 1),
('550e8400-e29b-41d4-a716-446655440002', '/packages/2x-n5090-64-specs.svg', 'Dual GPU Specifications', 'Complete technical specifications for the dual N5090 setup', 2),

-- Pro Server
('550e8400-e29b-41d4-a716-446655440003', '/packages/2x-h100-160-hero.svg', 'Enterprise H100 Server', 'Enterprise-grade dual H100 configuration for AI training', 1),
('550e8400-e29b-41d4-a716-446655440003', '/packages/2x-h100-160-specs.svg', 'Enterprise Specifications', 'Complete enterprise hardware specifications and capabilities', 2);

-- Make sku unique and not null after populating
alter table packages alter column sku set not null;
alter table packages add constraint packages_sku_unique unique (sku);

-- Create index for better query performance
create index idx_package_images_package_id on package_images (package_id, sort_order);

-- users & orgs
create table organizations (
  id uuid primary key,
  name text not null unique,
  created_at timestamptz not null default now()
);

create table users (
  id uuid primary key,
  org_id uuid not null references organizations(id) on delete cascade,
  email citext not null unique,
  pwd_hash text not null,
  created_at timestamptz not null default now(),
  is_admin boolean not null default false
);

-- orders & servers
create type gpu_class as enum ('None','L4','A100_40G','A100_80G','H100_80G','RTX_4090','RTX_5090');

create table server_orders (
  id uuid primary key,
  org_id uuid not null references organizations(id) on delete cascade,
  plan_cpu_cores smallint not null,
  plan_ram_gb smallint not null,
  plan_storage_gb integer not null,
  plan_gpu gpu_class not null,
  pq_enabled boolean not null default true,
  notes text,
  status text not null default 'queued',  -- queued|provisioning|active|failed|cancelled
  created_at timestamptz not null default now()
);

create table servers (
  id uuid primary key,
  org_id uuid not null references organizations(id) on delete cascade,
  order_id uuid references server_orders(id),
  hostname text not null unique,
  public_ip inet,
  specs jsonb not null,
  status text not null default 'provisioning',
  created_at timestamptz not null default now()
);

-- deployments
create table deployments (
  id uuid primary key,
  server_id uuid not null references servers(id) on delete cascade,
  kind text not null,               -- e.g. "vLLM","TGI","Ollama"
  config jsonb not null,            -- model, quantization, limits
  status text not null default 'pending',
  created_at timestamptz not null default now()
);

-- packages
create table packages (
  id uuid primary key,
  name text not null unique,
  description text not null,
  hardware_description text not null,
  cpu_cores smallint not null,
  ram_gb smallint not null,
  storage_gb integer not null,
  gpu_class gpu_class not null,
  gpu_count smallint not null default 1,
  vram_gb smallint not null default 0,
  setup_price_usdc integer not null,
  monthly_price_usdc integer not null,
  is_active boolean not null default true,
  created_at timestamptz not null default now()
);

-- insert default packages
insert into packages (id, name, description, hardware_description, cpu_cores, ram_gb, storage_gb, gpu_class, gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc) values
('550e8400-e29b-41d4-a716-446655440001', 'Midrange Consumer', 'Perfect for development and small-scale inference workloads', 'GMKtek X2 (or similar) with integrated GPU and shared system RAM', 16, 64, 2000, 'None', 0, 0, 3000, 200),
('550e8400-e29b-41d4-a716-446655440002', 'Top Consumer', 'High-performance setup for demanding AI applications', 'Ryzen 9950, 64GB RAM, dual 32GB RTX 5090s, liquid cooling', 16, 64, 4000, 'RTX_5090', 2, 64, 20000, 500),
('550e8400-e29b-41d4-a716-446655440003', 'Pro Server', 'Enterprise-grade AI infrastructure for production workloads', 'Dual H100 80GB with enterprise cooling and redundancy', 32, 128, 8000, 'H100_80G', 2, 160, 100000, 1000);

-- audit
create table audit_log (
  id bigserial primary key,
  org_id uuid,
  actor_user_id uuid,
  action text not null,
  details jsonb,
  at timestamptz not null default now()
);

#!/bin/bash

# Database setup script for Qapish AI Colocation Platform
set -e

echo "🚀 Setting up Qapish database..."

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL not found. Please install PostgreSQL first."
    echo "   Fedora: sudo dnf install postgresql postgresql-server postgresql-contrib"
    echo "   RHEL/CentOS Stream: sudo dnf install postgresql postgresql-server postgresql-contrib"
    echo "   macOS: brew install postgresql"
    echo "   Or use Podman: ./script/podman-db.sh"
    exit 1
fi

# Default database settings
DB_NAME="qapish"
DB_USER="postgres"
DB_HOST="localhost"
DB_PORT="5432"

# Check if running as root or if PostgreSQL service is running
if ! pg_isready -h $DB_HOST -p $DB_PORT &> /dev/null; then
    echo "❌ PostgreSQL service is not running. Please start it first:"
    echo "   Fedora/RHEL/CentOS Stream: sudo systemctl enable --now postgresql"
    echo "   Note: You may need to initialize the database first:"
    echo "   sudo postgresql-setup --initdb"
    echo "   macOS: brew services start postgresql"
    exit 1
fi

# Create database if it doesn't exist
if ! sudo -u postgres psql -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
    echo "📊 Creating database '$DB_NAME'..."
    sudo -u postgres createdb $DB_NAME
    echo "✅ Database '$DB_NAME' created successfully"
else
    echo "✅ Database '$DB_NAME' already exists"
fi

# Set up environment variables
DATABASE_URL="postgresql://$DB_USER:password@$DB_HOST:$DB_PORT/$DB_NAME"
echo "DATABASE_URL=$DATABASE_URL" > .env

echo "📝 Environment file created: .env"
echo "🔑 Database URL: $DATABASE_URL"

# Run migrations
echo "🔧 Running database migrations..."
export DATABASE_URL=$DATABASE_URL

# Create the tables directly using psql
sudo -u postgres psql -d $DB_NAME << 'EOF'
-- users & orgs
CREATE TABLE IF NOT EXISTS organizations (
  id uuid PRIMARY KEY,
  name text NOT NULL UNIQUE,
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY,
  org_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  email citext NOT NULL UNIQUE,
  pwd_hash text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT NOW(),
  is_admin boolean NOT NULL DEFAULT FALSE
);

-- Enable citext extension if not exists
CREATE EXTENSION IF NOT EXISTS citext;

-- orders & servers
DO $$ BEGIN
    CREATE TYPE gpu_class AS ENUM ('None','L4','A100_40G','A100_80G','H100_80G','RTX_4090','RTX_5090');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    ALTER TYPE gpu_class ADD VALUE 'Radeon_8060S';
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS server_orders (
  id uuid PRIMARY KEY,
  org_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  plan_cpu_cores smallint NOT NULL,
  plan_ram_gb smallint NOT NULL,
  plan_storage_gb integer NOT NULL,
  plan_gpu gpu_class NOT NULL,
  pq_enabled boolean NOT NULL DEFAULT TRUE,
  notes text,
  status text NOT NULL DEFAULT 'queued',
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS servers (
  id uuid PRIMARY KEY,
  org_id uuid NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  order_id uuid REFERENCES server_orders(id),
  hostname text NOT NULL UNIQUE,
  public_ip inet,
  specs jsonb NOT NULL,
  status text NOT NULL DEFAULT 'provisioning',
  created_at timestamptz NOT NULL DEFAULT NOW()
);

-- deployments
CREATE TABLE IF NOT EXISTS deployments (
  id uuid PRIMARY KEY,
  server_id uuid NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
  kind text NOT NULL,
  config jsonb NOT NULL,
  status text NOT NULL DEFAULT 'pending',
  created_at timestamptz NOT NULL DEFAULT NOW()
);

-- packages
CREATE TABLE IF NOT EXISTS packages (
  id uuid PRIMARY KEY,
  name text NOT NULL UNIQUE,
  description text NOT NULL,
  hardware_description text NOT NULL,
  cpu_cores smallint NOT NULL,
  ram_gb smallint NOT NULL,
  storage_gb integer NOT NULL,
  gpu_class gpu_class NOT NULL,
  gpu_count smallint NOT NULL DEFAULT 1,
  vram_gb smallint NOT NULL DEFAULT 0,
  setup_price_usdc integer NOT NULL,
  monthly_price_usdc integer NOT NULL,
  is_active boolean NOT NULL DEFAULT TRUE,
  created_at timestamptz NOT NULL DEFAULT NOW()
);

-- insert default packages (only if table is empty)
INSERT INTO packages (id, name, description, hardware_description, cpu_cores, ram_gb, storage_gb, gpu_class, gpu_count, vram_gb, setup_price_usdc, monthly_price_usdc)
SELECT * FROM (VALUES
('550e8400-e29b-41d4-a716-446655440001'::uuid, 'Midrange Consumer', 'Perfect for development and small-scale inference workloads', 'GMKtek X2 (or similar) with integrated GPU and shared system RAM', 16, 64, 2000, 'None'::gpu_class, 0, 0, 3000, 200),
('550e8400-e29b-41d4-a716-446655440002'::uuid, 'Top Consumer', 'High-performance setup for demanding AI applications', 'Ryzen 9950, 64GB RAM, dual 32GB RTX 5090s, liquid cooling', 16, 64, 4000, 'RTX_5090'::gpu_class, 2, 64, 20000, 500),
('550e8400-e29b-41d4-a716-446655440003'::uuid, 'Pro Server', 'Enterprise-grade AI infrastructure for production workloads', 'Dual H100 80GB with enterprise cooling and redundancy', 32, 128, 8000, 'H100_80G'::gpu_class, 2, 160, 100000, 1000)
) AS new_packages
WHERE NOT EXISTS (SELECT 1 FROM packages);

-- audit
CREATE TABLE IF NOT EXISTS audit_log (
  id bigserial PRIMARY KEY,
  org_id uuid,
  actor_user_id uuid,
  action text NOT NULL,
  details jsonb,
  at timestamptz NOT NULL DEFAULT NOW()
);

-- Create a demo organization for testing
INSERT INTO organizations (id, name)
SELECT '550e8400-e29b-41d4-a716-446655440000'::uuid, 'Demo Organization'
WHERE NOT EXISTS (SELECT 1 FROM organizations WHERE id = '550e8400-e29b-41d4-a716-446655440000'::uuid);

EOF

echo "✅ Database migrations completed successfully!"
echo ""
echo "🎉 Database setup complete!"
echo "   • Database: $DB_NAME"
echo "   • Connection: $DATABASE_URL"
echo "   • Default packages: 3 AI colocation packages added"
echo "   • Demo organization created for testing"
echo ""
echo "🚀 You can now start the development server with:"
echo "   just dev"
echo ""
echo "Or run the individual components:"
echo "   just dev-api    # API server on :8081"
echo "   just dev-web    # Web frontend on :8080"

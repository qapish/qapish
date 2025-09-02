# Qapish AI Colocation Platform

🚀 **Defensive AI Colocation** - Deploy enterprise-grade AI infrastructure in the secure Balkan mountains with post-quantum cryptography.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/postgresql-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![WebAssembly](https://img.shields.io/badge/webassembly-%23654FF0.svg?style=for-the-badge&logo=webassembly&logoColor=white)
![Fedora](https://img.shields.io/badge/fedora-294172.svg?style=for-the-badge&logo=fedora&logoColor=white)
![Podman](https://img.shields.io/badge/podman-892CA0.svg?style=for-the-badge&logo=podman&logoColor=white)

## 🌟 Features

- **🔐 Post-Quantum Security** - Future-proof encryption protecting against quantum computing attacks
- **🏔️ Secure Datacenter** - Located in geopolitically stable Balkan mountains with physical security
- **⚡ High-Performance Hardware** - RTX 4090/5090s, H100 80GB GPUs with enterprise cooling
- **🧠 Custom Inference Engines** - Pre-configured vLLM, TGI, and Ollama setups
- **🔄 Dynamic Model Loading** - Hot-swap LLMs optimized for your specific hardware
- **📊 Full-Stack Monitoring** - 24/7 monitoring with comprehensive dashboards
- **💰 USDC Pricing** - Transparent cryptocurrency-based pricing

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web Frontend  │    │   API Server    │    │   PostgreSQL    │
│   (Leptos/WASM) │◄──►│   (Axum/Rust)   │◄──►│   Database      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                        │                        │
         │                        ▼                        │
         │              ┌─────────────────┐                │
         └──────────────►│ Infrastructure  │◄───────────────┘
                        │ Management      │
                        └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites (Fedora)

This project is developed and deployed on **Fedora Linux** for optimal compatibility with modern tooling and post-quantum cryptography libraries.

- **Fedora 38+** (Workstation, Server, or CoreOS)
- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Development tools** - `sudo dnf groupinstall "Development Tools"`
- **Container runtime** - `sudo dnf install podman` (rootless containers)
- **Database** - `sudo dnf install postgresql postgresql-server` (or use Podman)
- **Build tools** - `cargo install just trunk`

For comprehensive setup instructions, see [`.docs/fedora-setup.md`](.docs/fedora-setup.md).

### Quick Start (Fedora)

```bash
# 1. Install system dependencies
sudo dnf install -y rust cargo podman postgresql-devel pkg-config openssl-devel

# 2. Clone the repository
git clone https://github.com/yourusername/qapish.git
cd qapish

# 3. Set up database with Podman (recommended)
./script/podman-db.sh

# 4. Install Rust tools
cargo install just trunk

# 5. Start development servers
just dev
```

### Alternative: Native PostgreSQL Setup

```bash
# Install PostgreSQL on Fedora
sudo dnf install postgresql postgresql-server postgresql-contrib
sudo postgresql-setup --initdb
sudo systemctl enable --now postgresql

# Set up local database
./script/setup-db.sh

# Start development
just dev
```

## 💻 Development (Fedora)

### Available Commands

```bash
# 🚀 Main development (API + Frontend)  
just dev

# 🔧 Individual components
just dev-api          # API server on :8081
just dev-web          # Web frontend on :8080

# 🗄️ Database management (Podman-based)
just db-podman        # Set up Podman PostgreSQL container
just db-setup         # Set up native PostgreSQL (Fedora)
just db-stop          # Stop Podman containers

# 📦 Building
just build            # Build everything for production
just build-web        # Build only web frontend
just prod             # Run production server

# 🧪 Testing & Quality
just test             # Run tests
just check            # Format and lint check  
just fmt              # Format code
just fix              # Fix common issues

# 🎬 Demo mode
just demo             # Quick demo without database
```

### Fedora-Specific Features
- **Post-quantum cryptography**: Native liboqs integration
- **Rootless containers**: Security-first Podman deployment
- **SystemD integration**: Quadlet service management
- **SELinux compatibility**: Container security policies

### Project Structure

```
qapish/
├── ai/                     # Core AI types and models
├── api/                    # REST API server (Axum)
├── web/                    # Frontend application (Leptos)
├── infra/                  # Infrastructure management
├── libs/
│   └── persistence/        # Database layer (SQLx)
├── script/                 # Setup and utility scripts
└── README.md
```

## 📦 AI Colocation Packages

### Midrange Consumer - $3,000 setup + $200/mo USDC
- **Hardware**: GMKtek X2 (or similar) with integrated GPU and shared system RAM
- **Specs**: 16 cores, 64GB RAM, 2TB NVMe, integrated graphics
- **Perfect for**: Development and small-scale inference workloads

### Top Consumer - $20,000 setup + $500/mo USDC  
- **Hardware**: Ryzen 9950, 64GB RAM, dual 32GB RTX 5090s, liquid cooling
- **Specs**: 16 cores, 64GB RAM, 4TB NVMe, 2x RTX 5090
- **Perfect for**: High-performance demanding AI applications

### Pro Server - $100,000 setup + $1,000/mo USDC
- **Hardware**: Dual H100 80GB with enterprise cooling and redundancy
- **Specs**: 32 cores, 128GB RAM, 8TB NVMe, 2x H100 80GB
- **Perfect for**: Enterprise-grade AI infrastructure for production workloads

All packages include:
- ✅ Custom inference engine (vLLM/TGI/Ollama)
- ✅ Pre-configured & tested LLMs
- ✅ Dynamic model loading
- ✅ 24/7 monitoring & support
- ✅ Post-quantum encryption
- ✅ Secure Balkan datacenter

## 🔐 Security Features

### Post-Quantum Cryptography
- **Algorithm**: CRYSTALS-Kyber for key encapsulation
- **Digital Signatures**: CRYSTALS-Dilithium
- **Hash Functions**: SHA-3 family
- **Future-Proof**: Resistant to quantum computer attacks

### Physical Security
- **Location**: Secure facilities in the Balkan mountains
- **Access Control**: Multi-factor authentication and biometrics
- **Monitoring**: 24/7 surveillance and intrusion detection
- **Power**: Redundant power systems with backup generators

### Network Security
- **Isolation**: Each deployment in isolated network segments
- **Encryption**: All traffic encrypted with post-quantum algorithms
- **Monitoring**: Real-time network traffic analysis
- **DDoS Protection**: Advanced mitigation systems

## 🛠️ API Reference

### Authentication
```bash
POST /api/auth/signup
POST /api/auth/login
```

### Packages
```bash
GET /api/packages          # List all available packages
```

### Orders
```bash
GET /api/orders            # List user orders
POST /api/orders           # Create new server order
```

### Health Check
```bash
GET /api/health            # Service health status
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## 📊 Database Schema

The platform uses PostgreSQL with the following main tables:

- **organizations** - Multi-tenant organization management
- **users** - User authentication and authorization
- **packages** - AI colocation package definitions
- **server_orders** - Customer server orders and provisioning
- **servers** - Active server instances and configurations
- **deployments** - AI model deployments and configurations
- **audit_log** - Comprehensive audit trail

## 🚀 Deployment

### Production Build

```bash
# Build optimized release
just build

# Set environment variables
export DATABASE_URL="postgresql://user:pass@host:5432/qapish"
export PORT=8080

# Run production server
just prod
```

### Container Deployment (Fedora + Podman)

The platform uses **Fedora-based container images** with **rootless Podman** for security and modern container practices:

```bash
# Build image using Fedora base
podman build -t qapish-api:latest -f Containerfile .

# Run with rootless Podman
podman run -p 8080:8080 -e DATABASE_URL="..." qapish-api:latest

# Production deployment with systemd + Quadlet
mkdir -p ~/.config/containers/systemd
cp orchestration/quadlet/* ~/.config/containers/systemd/
systemctl --user daemon-reload
systemctl --user enable --now qapish-postgres.service qapish-api.service
```

See [`.docs/quadlet-deployment.md`](.docs/quadlet-deployment.md) for complete deployment instructions.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Run the test suite (`just test`)
6. Format your code (`just fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [docs.qapish.com](https://docs.qapish.com)
- **Issues**: [GitHub Issues](https://github.com/yourusername/qapish/issues)
- **Discord**: [Join our community](https://discord.gg/qapish)
- **Email**: support@qapish.com

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Leptos](https://leptos.dev/)
- Styled with [Leptonic](https://github.com/lpotthast/leptonic) UI components
- Database powered by [PostgreSQL](https://www.postgresql.org/) and [SQLx](https://github.com/launchbadge/sqlx)
- Container runtime by [Podman](https://podman.io/) with rootless security
- Deployed on [Fedora Linux](https://getfedora.org/) for modern tooling
- Post-quantum cryptography via [liboqs](https://github.com/open-quantum-safe/liboqs) (available in Fedora)
- Service orchestration with [systemd](https://systemd.io/) and Quadlet

---

<div align="center">
  <strong>🔒 Secured by Post-Quantum Cryptography</strong><br>
  <strong>🐧 Powered by Fedora Linux + Podman</strong><br>
  Built with ❤️ for the future of AI infrastructure
</div>
use leptonic::components::prelude::{LeptonicTheme, Root};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

fn api_base() -> String {
    if cfg!(debug_assertions) {
        "http://localhost:8081".to_string()
    } else {
        "".to_string() // Use relative URLs in production
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct PackageImage {
    filename: String,
    title: String,
    description: String,
}

#[derive(Clone, Serialize, Deserialize)]
enum Availability {
    Preorder,
    InStock,
    Build { hours: u16 },
}

#[derive(Clone, Serialize, Deserialize)]
enum Provenance {
    New,
    Used { hours: u32 },
}

#[derive(Clone, Serialize, Deserialize)]
struct Package {
    id: String,
    name: String,
    sku: String,
    description: String,
    hardware_description: String,
    cpu_cores: u16,
    ram_gb: u16,
    storage_gb: u32,
    gpu_class: String,
    gpu_count: u16,
    vram_gb: u16,
    setup_price_usdc: u32,
    monthly_price_usdc: u32,
    images: Vec<PackageImage>,
    availability: Availability,
    provenance: Provenance,
}

#[component]
fn Landing() -> impl IntoView {
    let packages = create_resource(
        || (),
        |_| async move {
            let url = format!("{}/api/packages", api_base());
            let resp = gloo_net::http::Request::get(&url).send().await;
            match resp {
                Ok(r) if r.status() == 200 => r.json::<Vec<Package>>().await.ok(),
                _ => None,
            }
        },
    );

    view! {
        <div class="landing-container">
            {/* Hero Section */}
            <section class="hero">
                <div class="hero-content">
                    <div class="hero-brand">
                        <img src="/logo/logo-wordmark.svg" alt="Qapish" class="qp-wordmark-logo" />
                        <div class="qp-domain-lockup">
                            "qapi"<span class="dot">"·"</span>"sh"
                        </div>
                    </div>
                    <div class="hero-badge">
                        "🔒 Post-Quantum Secured"
                    </div>
                    <h1 class="hero-title qp-hero-title">
                        "Defensive AI Colocation"
                    </h1>
                    <p class="hero-subtitle">
                        "PQ‑secured managed AI servers in the Balkan mountains."
                    </p>
                    <div class="hero-features">
                        <div class="feature-item">
                            <span class="feature-icon">"🛡️"</span>
                            <span>"Post‑quantum security, end‑to‑end"</span>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">"🔧"</span>
                            <span>"Managed inference platforms and models"</span>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">"💎"</span>
                            <span>"Transparent pricing, EU jurisdiction"</span>
                        </div>
                    </div>
                    <div class="hero-cta">
                        <A href="/signup" class="cta-button primary">"Order a server"</A>
                        <A href="mailto:hello@qapi.sh" class="cta-button secondary">"Talk to engineering"</A>
                    </div>
                </div>
                <div class="hero-visual">
                    <div class="gpu-visualization">
                        <div class="gpu-card gpu-primary"></div>
                        <div class="gpu-card gpu-secondary"></div>
                        <div class="connection-lines"></div>
                    </div>
                </div>
            </section>

            {/* Packages Section */}
            <section class="packages-section">
                <div class="section-header">
                    <h2 class="section-title">"Secure Colocation for Serious AI"</h2>
                    <p class="section-subtitle">
                        "From metal to model: inference platforms and LLMs, curated and automated."
                    </p>
                </div>

                <div class="packages-grid">
                    <Suspense fallback=move || view! {
                        <div class="loading-packages">
                            <div class="spinner"></div>
                            <p>"Loading packages..."</p>
                        </div>
                    }>
                        {move || {
                            packages.get().map(|pkgs| match pkgs {
                                Some(packages) => {
                                    packages.into_iter().enumerate().map(|(idx, pkg)| {
                                        let is_popular = idx == 1; // Make middle package popular
                                        let card_class = if is_popular { "package-card popular" } else { "package-card" };

                                        view! {
                                            <div class=card_class>
                                                {if is_popular {
                                                    view! { <div class="popular-badge">"Most Popular"</div> }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}

                                                <div class="package-header">
                                                    <h3 class="package-name">{pkg.name.clone()}</h3>
                                                    <p class="package-description">{pkg.description.clone()}</p>
                                                </div>

                                                <div class="package-pricing">
                                                    <div class="price-setup">
                                                        <span class="price-label">"Hardware & Setup"</span>
                                                        <span class="price-amount">
                                                            "$" {pkg.setup_price_usdc.to_string()} " USDC"
                                                        </span>
                                                    </div>
                                                    <div class="price-monthly">
                                                        <span class="price-label">"Monthly Hosting"</span>
                                                        <span class="price-amount primary">
                                                            "$" {pkg.monthly_price_usdc} " USDC/mo"
                                                        </span>
                                                    </div>
                                                </div>

                                                <div class="package-availability">
                                                    <div class="availability-item">
                                                        <span class="availability-icon">
                                                            {match &pkg.availability {
                                                                Availability::InStock => "✅",
                                                                Availability::Preorder => "📋",
                                                                Availability::Build { .. } => "🔧",
                                                            }}
                                                        </span>
                                                        <span>
                                                            {match &pkg.availability {
                                                                Availability::InStock => "In Stock".to_string(),
                                                                Availability::Preorder => "Preorder".to_string(),
                                                                Availability::Build { hours } => format!("{}h Build", hours),
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="provenance-item">
                                                        <span class="provenance-icon">
                                                            {match &pkg.provenance {
                                                                Provenance::New => "🆕",
                                                                Provenance::Used { .. } => "♻️",
                                                            }}
                                                        </span>
                                                        <span>
                                                            {match &pkg.provenance {
                                                                Provenance::New => "New".to_string(),
                                                                Provenance::Used { hours } => format!("Used ({}h)", hours),
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>

                                                <div class="package-specs">
                                                    <div class="spec-item">
                                                        <span class="spec-icon">"🧠"</span>
                                                        <span>{pkg.cpu_cores} " cores, " {pkg.ram_gb} "GB RAM"</span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-icon">"🎮"</span>
                                                        <span>{pkg.gpu_count} "x " {pkg.gpu_class.replace("_", " ")}
                                                            {if pkg.vram_gb > 0 {
                                                                format!(" ({}GB VRAM)", pkg.vram_gb)
                                                            } else {
                                                                String::new()
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-icon">"💾"</span>
                                                        <span>{format!("{:.1}TB", pkg.storage_gb as f32 / 1000.0)} " NVMe Storage"</span>
                                                    </div>
                                                </div>

                                                <div class="hardware-details">
                                                    <h4>"Hardware Configuration"</h4>
                                                    <p>{pkg.hardware_description.clone()}</p>
                                                </div>

                                                <div class="package-includes">
                                                    <h4>"Included Services"</h4>
                                                    <ul>
                                                        <li>"✅ Custom inference engine (vLLM/TGI/Ollama)"</li>
                                                        <li>"✅ Pre-configured & tested LLMs"</li>
                                                        <li>"✅ Dynamic model loading"</li>
                                                        <li>"✅ 24/7 monitoring & support"</li>
                                                        <li>"✅ Post-quantum encryption"</li>
                                                        <li>"✅ Secure Balkan datacenter"</li>
                                                    </ul>
                                                </div>

                                                <A href={format!("/package/{}", pkg.sku)} class="cta-button">
                                                    "Deploy Now →"
                                                </A>
                                            </div>
                                        }.into_view()
                                    }).collect::<Vec<_>>().into_view()
                                },
                                None => view! {
                                    <div class="error-state">
                                        <p>"Failed to load packages. Please try again later."</p>
                                    </div>
                                }.into_view()
                            })
                        }}
                    </Suspense>
                </div>
            </section>

            {/* Security Section */}
            <section class="security-section">
                <div class="security-content">
                    <h2>"Security, Now and Later"</h2>
                    <div class="security-features">
                        <div class="security-item">
                            <div class="security-icon">"🔐"</div>
                            <h3>"Post‑Quantum Cryptography"</h3>
                            <p>"Defense‑in‑depth, zero‑trust edge. Future-proof encryption ready for quantum threats."</p>
                        </div>
                        <div class="security-item">
                            <div class="security-icon">"🏔️"</div>
                            <h3>"Balkan Mountain Location"</h3>
                            <p>"Resilient power, cold climate advantage, EU jurisdiction. Place matters."</p>
                        </div>
                        <div class="security-item">
                            <div class="security-icon">"👑"</div>
                            <h3>"Sovereignty & Privacy"</h3>
                            <p>"Your compute, your keys, your isolation. Complete ownership and control."</p>
                        </div>
                    </div>
                </div>
            </section>

            {/* CTA Section */}
            <section class="final-cta">
                <h2>"Performance & Reliability"</h2>
                <p>"Low‑noise facilities, proactive monitoring, SLA backed. It's obvious and it works."</p>
                <A href="/signup" class="cta-button primary large">"Order a server"</A>
            </section>

            {/* Footer */}
            <footer class="footer">
                <div class="footer-content">
                    <div class="footer-brand">
                        <span class="qp-wordmark">"Qapish"</span>
                        <span class="pronunciation">" (kah‑PEESH)"</span>
                    </div>
                    <p class="footer-tagline">"Secure colocation for serious AI."</p>
                    <div class="footer-links">
                        <a href="mailto:hello@qapi.sh">"hello@qapi.sh"</a>
                        <span>" • "</span>
                        <a href="mailto:support@qapi.sh">"support@qapi.sh"</a>
                        <span>" • "</span>
                        <a href="mailto:security@qapi.sh">"security@qapi.sh"</a>
                    </div>
                </div>
            </footer>
        </div>

        <style>
            {r#"
            .landing-container {
                min-height: 100vh;
                background: var(--qp-surface);
                color: var(--qp-text);
                font-family: var(--qp-font-ui);
            }

            @media (prefers-color-scheme: dark) {
                .landing-container {
                    background: linear-gradient(135deg, var(--qp-surface-dark) 0%, var(--qp-surface-dark) 100%);
                    color: var(--qp-text-dark);
                }
            }

            .hero {
                display: flex;
                align-items: center;
                min-height: 100vh;
                padding: 0 2rem;
                max-width: 1200px;
                margin: 0 auto;
                gap: 4rem;
            }

            .hero-brand {
                margin-bottom: 2rem;
            }

            .hero-brand .qp-wordmark {
                font-size: 2rem;
                display: block;
                margin-bottom: 0.5rem;
            }

            .hero-brand .qp-domain-lockup {
                font-size: 1.25rem;
                color: var(--qp-cyan);
            }

            .hero-cta {
                display: flex;
                gap: 1rem;
                margin-top: 2rem;
            }

            .cta-button.secondary {
                background: transparent;
                border: 2px solid var(--qp-cyan);
                color: var(--qp-cyan);
            }

            .cta-button.secondary:hover {
                background: var(--qp-cyan);
                color: white;
            }

            .hero-content {
                flex: 1;
                max-width: 600px;
            }

            .hero-badge {
                display: inline-block;
                padding: 0.5rem 1rem;
                background: var(--qp-cyan);
                border-radius: 50px;
                font-size: 0.875rem;
                font-weight: 600;
                margin-bottom: 1.5rem;
                color: white;
            }

            .hero-title {
                font-size: 3.5rem;
                font-weight: 800;
                line-height: 1.1;
                margin-bottom: 1.5rem;
            }

            .hero-subtitle {
                font-size: 1.25rem;
                line-height: 1.6;
                color: var(--qp-text);
                margin-bottom: 2rem;
                opacity: 0.8;
            }

            @media (prefers-color-scheme: dark) {
                .hero-subtitle {
                    color: var(--qp-text-dark);
                }
            }

            .hero-features {
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }

            .feature-item {
                display: flex;
                align-items: center;
                gap: 0.75rem;
                font-weight: 500;
            }

            .feature-icon {
                font-size: 1.5rem;
            }

            .hero-visual {
                flex: 1;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 400px;
            }

            .gpu-visualization {
                position: relative;
                width: 300px;
                height: 200px;
            }

            .gpu-card {
                position: absolute;
                width: 120px;
                height: 80px;
                background: linear-gradient(135deg, var(--qp-cyan) 0%, var(--qp-ink) 100%);
                border-radius: 8px;
                box-shadow: 0 8px 32px rgba(6, 182, 212, 0.3);
                animation: float 6s ease-in-out infinite;
            }

            .gpu-primary {
                top: 20px;
                left: 20px;
                animation-delay: 0s;
            }

            .gpu-secondary {
                top: 60px;
                right: 20px;
                animation-delay: -3s;
            }

            @keyframes float {
                0%, 100% { transform: translateY(0px); }
                50% { transform: translateY(-20px); }
            }

            /* Package navigation styles */

            .package-counter {
                color: var(--qp-text-dark);
                font-weight: 500;
                opacity: 0.7;
                font-size: 0.875rem;
                white-space: nowrap;
            }

            .package-nav-top {
                margin-bottom: 1.5rem;
            }

            .package-nav-top .bottom-nav-simple {
                justify-content: space-between;
                max-width: 300px;
            }

            .package-nav-bottom {
                margin-top: 2rem;
                padding: 1.5rem 0;
                border-top: 1px solid rgba(226, 232, 240, 0.1);
            }

            .bottom-nav-simple {
                display: flex;
                justify-content: center;
                align-items: center;
                gap: 3rem;
                width: 100%;
                max-width: 400px;
                margin: 0 auto;
            }

            .nav-link-simple {
                color: var(--qp-cyan);
                text-decoration: none;
                font-weight: 500;
                font-size: 0.9rem;
                padding: 0.5rem 0;
                transition: opacity 0.2s ease;
                opacity: 0.8;
            }

            .nav-link-simple:hover {
                opacity: 1;
                text-decoration: underline;
            }

            .back-link-center {
                color: var(--qp-text-dark);
                text-decoration: none;
                font-weight: 500;
                font-size: 0.9rem;
                opacity: 0.7;
                transition: all 0.2s ease;
            }

            .back-link-center:hover {
                color: var(--qp-cyan);
                opacity: 1;
            }

            /* Responsive navigation */
            @media (max-width: 768px) {
                .bottom-nav-simple {
                    flex-direction: column;
                    gap: 1rem;
                    max-width: none;
                }

                .package-counter {
                    font-size: 0.8rem;
                    order: -1;
                }
            }

            /* Dark theme support */
            @media (prefers-color-scheme: dark) {
                .package-counter {
                    color: var(--qp-text-dark);
                }

                .nav-link-simple {
                    color: var(--qp-cyan);
                }

                .back-link-center {
                    color: var(--qp-text-dark);
                }

                .back-link-center:hover {
                    color: var(--qp-cyan);
                }
            }

            .dark .package-counter {
                color: var(--qp-text-dark);
            }

            .dark .nav-link-simple {
                color: var(--qp-cyan);
            }

            .dark .back-link-center {
                color: var(--qp-text-dark);
            }

            .dark .back-link-center:hover {
                color: var(--qp-cyan);
            }

            /* Package Grid Styles */
            .packages-section {
                padding: 4rem 2rem;
                max-width: 1400px;
                margin: 0 auto;
            }

            .section-header {
                text-align: center;
                margin-bottom: 4rem;
            }

            .section-title {
                font-size: 2.5rem;
                font-weight: 700;
                margin-bottom: 1rem;
                color: var(--qp-text-dark);
                text-align: center;
                text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
            }

            .section-subtitle {
                font-size: 1.125rem;
                color: var(--qp-text-dark);
                opacity: 0.9;
                max-width: 600px;
                margin: 0 auto;
                line-height: 1.6;
                text-align: center;
            }

            .packages-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
                gap: 2rem;
                margin-top: 3rem;
            }

            .package-card {
                background: var(--qp-surface-dark);
                border: 1px solid rgba(226, 232, 240, 0.2);
                border-radius: 16px;
                padding: 2rem;
                position: relative;
                transition: all 0.3s ease;
                box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
                color: var(--qp-text-dark);
            }

            .package-card:hover {
                transform: translateY(-4px);
                box-shadow: 0 12px 40px rgba(6, 182, 212, 0.2);
                border-color: var(--qp-cyan);
            }

            .package-card.popular {
                border: 2px solid var(--qp-cyan);
                transform: scale(1.05);
            }

            .popular-badge {
                position: absolute;
                top: -12px;
                left: 50%;
                transform: translateX(-50%);
                background: var(--qp-cyan);
                color: white;
                padding: 0.5rem 1.5rem;
                border-radius: 50px;
                font-size: 0.875rem;
                font-weight: 600;
            }

            .package-header {
                margin-bottom: 2rem;
            }

            .package-name {
                font-size: 1.5rem;
                font-weight: 700;
                margin-bottom: 0.5rem;
                color: var(--qp-cyan);
                text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
            }

            .package-description {
                color: var(--qp-text-dark);
                opacity: 0.9;
                line-height: 1.5;
            }

            .package-pricing {
                margin-bottom: 2rem;
                padding: 1.5rem;
                background: rgba(6, 182, 212, 0.15);
                border-radius: 12px;
                border: 2px solid var(--qp-cyan);
                box-shadow: 0 2px 8px rgba(6, 182, 212, 0.2);
            }

            .price-setup, .price-monthly {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 1rem;
                padding: 0.5rem;
                background: rgba(15, 23, 42, 0.3);
                border-radius: 0.5rem;
            }

            .price-monthly {
                margin-bottom: 0;
                padding-top: 0.75rem;
                border-top: 1px solid rgba(6, 182, 212, 0.2);
            }

            .price-label {
                color: var(--qp-text-dark);
                opacity: 0.8;
                font-size: 0.875rem;
            }

            .price-amount {
                font-size: 1.25rem;
                font-weight: 700;
            }

            .price-amount.primary {
                color: var(--qp-cyan);
                font-size: 1.5rem;
                font-weight: 800;
                text-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
            }

            .package-specs {
                margin-bottom: 2rem;
            }

            .spec-item {
                display: flex;
                align-items: center;
                gap: 0.75rem;
                margin-bottom: 0.75rem;
                font-size: 0.95rem;
                padding: 0.5rem;
                background: rgba(6, 182, 212, 0.1);
                border-radius: 0.375rem;
                color: var(--qp-text-dark);
                border: 1px solid rgba(6, 182, 212, 0.2);
            }

            .spec-icon {
                font-size: 1.25rem;
                width: 24px;
                text-align: center;
                flex-shrink: 0;
            }

            .hardware-details, .package-includes {
                margin-bottom: 2rem;
                padding: 1rem;
                background: rgba(15, 23, 42, 0.03);
                border-radius: 0.5rem;
                border-left: 3px solid var(--qp-cyan);
            }

            .hardware-details h4, .package-includes h4 {
                font-size: 1rem;
                font-weight: 600;
                margin-bottom: 0.75rem;
                color: var(--qp-cyan);
            }

            .hardware-details p {
                color: var(--qp-text-dark);
                opacity: 0.9;
                font-size: 0.9rem;
                line-height: 1.5;
                margin: 0;
            }

            .package-includes ul {
                list-style: none;
                padding: 0;
                margin: 0;
            }

            .package-includes li {
                font-size: 0.9rem;
                margin-bottom: 0.5rem;
                color: var(--qp-text-dark);
                opacity: 0.9;
                display: flex;
                align-items: center;
                gap: 0.5rem;
            }

            .cta-button {
                width: 100%;
                padding: 1rem 2rem;
                background: var(--qp-cyan);
                color: white;
                border: 2px solid var(--qp-cyan);
                border-radius: 8px;
                font-size: 1rem;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.3s ease;
                text-decoration: none;
                display: inline-block;
                text-align: center;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            }

            @media (prefers-color-scheme: dark) {
                .cta-button {
                    background: var(--qp-cyan);
                    color: var(--qp-surface-dark);
                    border-color: var(--qp-cyan);
                    font-weight: 700;
                }

                .cta-button:hover {
                    background: #0891b2;
                    color: white;
                }

                .spec-item {
                    background: rgba(6, 182, 212, 0.1);
                    color: var(--qp-text-dark);
                }

                .hardware-details, .package-includes {
                    background: rgba(226, 232, 240, 0.05);
                }

                .hardware-details p, .package-includes li {
                    color: var(--qp-text-dark);
                }

                .section-title {
                    color: var(--qp-text-dark);
                }

                .section-subtitle {
                    color: var(--qp-text-dark);
                }
            }

            .cta-button:hover {
                transform: translateY(-2px);
                box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
            }

            .cta-button.primary {
                background: var(--qp-cyan);
                border-color: var(--qp-cyan);
            }

            .cta-button.primary:hover {
                background: var(--qp-cyan);
                opacity: 0.9;
                border-color: var(--qp-cyan);
            }

            .cta-button.large {
                padding: 1.25rem 3rem;
                font-size: 1.125rem;
            }

            .security-section {
                padding: 6rem 2rem;
                background: var(--qp-surface-dark);
            }

            .security-content {
                max-width: 1200px;
                margin: 0 auto;
                text-align: center;
            }

            .security-content h2 {
                font-size: 2.5rem;
                font-weight: 700;
                margin-bottom: 3rem;
            }

            .security-features {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 3rem;
            }

            .security-item {
                text-align: center;
            }

            .security-icon {
                font-size: 3rem;
                margin-bottom: 1rem;
                display: block;
            }

            .security-item h3 {
                font-size: 1.5rem;
                font-weight: 600;
                margin-bottom: 1rem;
                color: var(--qp-cyan);
            }

            .security-item p {
                color: var(--qp-text-dark);
                opacity: 0.8;
                line-height: 1.6;
            }

            .final-cta {
                padding: 6rem 2rem;
                text-align: center;
                max-width: 800px;
                margin: 0 auto;
            }

            .final-cta h2 {
                font-size: 2.5rem;
                font-weight: 700;
                margin-bottom: 1rem;
            }

            .final-cta p {
                font-size: 1.125rem;
                color: var(--qp-text);
                opacity: 0.7;
                margin-bottom: 3rem;
                line-height: 1.6;
            }

            .footer {
                padding: 3rem 2rem 2rem;
                text-align: center;
                border-top: 1px solid var(--qp-ink-100);
                background: var(--qp-surface);
            }

            @media (prefers-color-scheme: dark) {
                .footer {
                    background: var(--qp-surface-dark);
                    border-top-color: var(--qp-ink-100);
                }
            }

            .footer-content {
                max-width: 1200px;
                margin: 0 auto;
            }

            .footer-brand {
                margin-bottom: 1rem;
            }

            .footer-brand .qp-wordmark {
                font-size: 1.5rem;
                font-weight: 900;
            }

            .footer-brand .pronunciation {
                font-size: 0.875rem;
                opacity: 0.7;
                font-style: italic;
            }

            .footer-tagline {
                margin-bottom: 1.5rem;
                font-size: 1rem;
                opacity: 0.8;
            }

            .footer-links {
                font-size: 0.875rem;
            }

            .footer-links a {
                color: var(--qp-cyan);
            }

            .loading-packages, .error-state {
                grid-column: 1 / -1;
                text-align: center;
                padding: 4rem 2rem;
            }

            .spinner {
                width: 40px;
                height: 40px;
                border: 4px solid var(--qp-ink-100);
                border-top: 4px solid var(--qp-cyan);
                border-radius: 50%;
                animation: spin 1s linear infinite;
                margin: 0 auto 1rem;
            }

            @media (prefers-color-scheme: dark) {
                .spinner {
                    border-color: var(--qp-ink-100);
                    border-top-color: var(--qp-cyan);
                }
            }

            @keyframes spin {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
            }

            @media (max-width: 768px) {
                .hero {
                    flex-direction: column;
                    text-align: center;
                    padding: 2rem 1rem;
                    min-height: auto;
                }

                .hero-title {
                    font-size: 2.5rem;
                }

                .hero-cta {
                    flex-direction: column;
                }

                .packages-grid {
                    grid-template-columns: 1fr;
                    gap: 1.5rem;
                }

                .package-card.popular {
                    transform: none;
                }

                .packages-section, .security-section, .final-cta {
                    padding: 3rem 1rem;
                }
            }
            "#}
        </style>
    }
}

#[component]
fn Signup() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (status, set_status) = create_signal(String::new());

    let submit = move |_| {
        let email = email.get();
        let password = password.get();
        wasm_bindgen_futures::spawn_local(async move {
            let body = serde_json::json!({ "email": email, "password": password });
            let url = format!("{}/api/auth/signup", api_base());
            let resp = gloo_net::http::Request::post(&url)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&body).unwrap())
                .unwrap()
                .send()
                .await;
            match resp {
                Ok(r) if r.status() == 200 => set_status.set("Signed up! 🎉".into()),
                Ok(r) => set_status.set(format!("Error {}", r.status())),
                Err(e) => set_status.set(format!("Network error: {e}")),
            }
        });
    };

    view! {
        <div style="max-width:480px;margin:2rem auto;padding:2rem;background:var(--qp-surface);border:1px solid var(--qp-ink-100);border-radius:16px;">
            <h2>"Create your account"</h2>
            <input
                placeholder="Email"
                style="width:100%;padding:1rem;margin:1rem 0;border:1px solid var(--qp-ink-100);border-radius:8px;background:var(--qp-surface);color:var(--qp-text);"
                on:input=move |e| set_email.set(event_target_value(&e))
            />
            <input
                type="password"
                placeholder="Password"
                style="width:100%;padding:1rem;margin:1rem 0;border:1px solid var(--qp-ink-100);border-radius:8px;background:var(--qp-surface);color:var(--qp-text);"
                on:input=move |e| set_password.set(event_target_value(&e))
            />
            <button
                class="cta-button primary"
                style="margin-top:1rem;"
                on:click=submit
            >
                "Sign up"
            </button>
            <p>{move || status.get()}</p>
        </div>
    }
}

#[component]
fn PackageDetail() -> impl IntoView {
    let params = use_params_map();
    let sku = move || params.with(|params| params.get("sku").cloned().unwrap_or_default());

    // Fetch current package
    let package_resource = create_resource(sku, |sku| async move {
        if sku.is_empty() {
            return None;
        }
        let url = format!("{}/api/packages/{}", api_base(), sku);
        let resp = gloo_net::http::Request::get(&url).send().await;
        match resp {
            Ok(r) if r.status() == 200 => r.json::<Package>().await.ok(),
            _ => None,
        }
    });

    // Fetch all packages for navigation
    let all_packages = create_resource(
        || (),
        |_| async move {
            let url = format!("{}/api/packages", api_base());
            let resp = gloo_net::http::Request::get(&url).send().await;
            match resp {
                Ok(r) if r.status() == 200 => r.json::<Vec<Package>>().await.ok(),
                _ => None,
            }
        },
    );

    view! {
        <div class="package-detail-container">
            <Suspense fallback=move || view! {
                <div class="loading">
                    <div class="spinner"></div>
                    <p>"Loading package details..."</p>
                </div>
            }>
                {move || {
                    // Get navigation info
                    let nav_info = move || {
                        if let (Some(Some(current_pkg)), Some(Some(all_pkgs))) = (package_resource.get(), all_packages.get()) {
                            let current_index = all_pkgs.iter().position(|p| p.sku == current_pkg.sku);
                            if let Some(index) = current_index {
                                let total = all_pkgs.len();
                                let prev_index = if index == 0 { total - 1 } else { index - 1 };
                                let next_index = if index == total - 1 { 0 } else { index + 1 };
                                Some((all_pkgs[prev_index].sku.clone(), all_pkgs[next_index].sku.clone(), index + 1, total))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };

                    package_resource.get().map(|pkg_opt| match pkg_opt {
                        Some(pkg) => {
                            let availability_text = match &pkg.availability {
                                Availability::InStock => "In Stock - Ships within 24 hours".to_string(),
                                Availability::Preorder => "Preorder - Ships when available".to_string(),
                                Availability::Build { hours } => format!("Custom Build - {} hour delivery", hours),
                            };

                            let provenance_text = match &pkg.provenance {
                                Provenance::New => "Brand New Hardware".to_string(),
                                Provenance::Used { hours } => format!("Refurbished - {} hours previous usage", hours),
                            };

                            let payment_policy = match &pkg.availability {
                                Availability::InStock | Availability::Build { .. } =>
                                    "Full payment required at time of order".to_string(),
                                Availability::Preorder =>
                                    "5% deposit on order, balance due when order confirmation and proforma invoice is issued, preceding build".to_string(),
                            };

                            view! {
                                <div class="package-detail">
                                    <div class="package-header">
                                        <div class="package-nav-top">
                                            {move || {
                                                nav_info().map(|(prev_sku, next_sku, current_num, total)| {
                                                    view! {
                                                        <div class="bottom-nav-simple">
                                                            <A href={format!("/package/{}", prev_sku)} class="nav-link-simple">
                                                                "← Previous"
                                                            </A>
                                                            <div class="package-counter">
                                                                <span>{current_num}" of "{total}</span>
                                                            </div>
                                                            <A href={format!("/package/{}", next_sku)} class="nav-link-simple">
                                                                "Next →"
                                                            </A>
                                                        </div>
                                                    }.into_view()
                                                }).unwrap_or_else(|| view! { <div></div> }.into_view())
                                            }}
                                        </div>
                                        <h1 class="package-title">{pkg.name.clone()}</h1>
                                        <div class="package-sku">SKU: {pkg.sku.clone()}</div>
                                    </div>

                                    <div class="package-content">
                                        <div class="package-images">
                                            {
                                                // Use fallback images if no images or placeholder URLs
                                                let display_images = if pkg.images.is_empty() || pkg.images.iter().any(|img| img.filename.contains("placehold.co")) {
                                                    match pkg.sku.as_str() {
                                                        "1x-a8060-96" => vec![
                                                            PackageImage {
                                                                filename: "/packages/1x-a8060-96-hero.svg".to_string(),
                                                                title: "AMD A8060 Server".to_string(),
                                                                description: "Professional AI compute server with 96GB HBM3e memory".to_string(),
                                                            },
                                                            PackageImage {
                                                                filename: "/packages/1x-a8060-96-specs.svg".to_string(),
                                                                title: "Technical Specifications".to_string(),
                                                                description: "Detailed hardware specifications and performance metrics".to_string(),
                                                            }
                                                        ],
                                                        "2x-n5090-64" => vec![
                                                            PackageImage {
                                                                filename: "/packages/2x-n5090-64-hero.svg".to_string(),
                                                                title: "Dual N5090 Configuration".to_string(),
                                                                description: "High-performance dual GPU setup with 64GB total VRAM".to_string(),
                                                            },
                                                            PackageImage {
                                                                filename: "/packages/2x-n5090-64-specs.svg".to_string(),
                                                                title: "Dual GPU Specifications".to_string(),
                                                                description: "Complete technical specifications for the dual N5090 setup".to_string(),
                                                            }
                                                        ],
                                                        "2x-h100-160" => vec![
                                                            PackageImage {
                                                                filename: "/packages/2x-h100-160-hero.svg".to_string(),
                                                                title: "Enterprise H100 Server".to_string(),
                                                                description: "Enterprise-grade dual H100 configuration for AI training".to_string(),
                                                            },
                                                            PackageImage {
                                                                filename: "/packages/2x-h100-160-specs.svg".to_string(),
                                                                title: "Enterprise Specifications".to_string(),
                                                                description: "Complete enterprise hardware specifications and capabilities".to_string(),
                                                            }
                                                        ],
                                                        _ => pkg.images.clone()
                                                    }
                                                } else {
                                                    pkg.images.clone()
                                                };

                                                display_images.iter().map(|img| {
                                                    view! {
                                                        <div class="package-image">
                                                            <img src={img.filename.clone()} alt={img.title.clone()} />
                                                            <div class="image-info">
                                                                <h3>{img.title.clone()}</h3>
                                                                <p>{img.description.clone()}</p>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect_view()
                                            }
                                        </div>

                                        <div class="package-info">
                                            <div class="package-description">
                                                <h2>"Description"</h2>
                                                <p>{pkg.description.clone()}</p>
                                            </div>

                                            <div class="package-specs">
                                                <h2>"Technical Specifications"</h2>
                                                <div class="spec-grid">
                                                    <div class="spec-item">
                                                        <span class="spec-label">"CPU Cores"</span>
                                                        <span class="spec-value">{pkg.cpu_cores}</span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-label">"RAM"</span>
                                                        <span class="spec-value">{pkg.ram_gb}" GB"</span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-label">"Storage"</span>
                                                        <span class="spec-value">{pkg.storage_gb}" GB"</span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-label">"GPU"</span>
                                                        <span class="spec-value">{format!("{:?}", pkg.gpu_class)}</span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-label">"GPU Count"</span>
                                                        <span class="spec-value">{pkg.gpu_count}</span>
                                                    </div>
                                                    <div class="spec-item">
                                                        <span class="spec-label">"VRAM"</span>
                                                        <span class="spec-value">{pkg.vram_gb}" GB"</span>
                                                    </div>
                                                </div>
                                                <div class="hardware-description">
                                                    <h3>"Hardware Details"</h3>
                                                    <p>{pkg.hardware_description.clone()}</p>
                                                </div>
                                            </div>

                                            <div class="package-availability">
                                                <h2>"Availability & Delivery"</h2>
                                                <div class="availability-info">
                                                    <div class="availability-status">{availability_text}</div>
                                                    <div class="provenance-info">{provenance_text}</div>
                                                </div>
                                            </div>

                                            <div class="package-pricing">
                                                <h2>"Pricing"</h2>
                                                <div class="pricing-details">
                                                    <div class="price-item">
                                                        <span class="price-label">"Hardware & Setup"</span>
                                                        <span class="price-amount">"$"{pkg.setup_price_usdc}" USDC"</span>
                                                    </div>
                                                    <div class="price-item">
                                                        <span class="price-label">"Monthly Hosting"</span>
                                                        <span class="price-amount">"$"{pkg.monthly_price_usdc}" USDC/mo"</span>
                                                    </div>
                                                </div>
                                                <div class="payment-policy">
                                                    <h3>"Payment Policy"</h3>
                                                    <p>{payment_policy}</p>
                                                </div>
                                            </div>

                                            <div class="package-actions">
                                                <A href="/signup" class="package-cta primary">
                                                    "Deploy Now"
                                                </A>
                                                <A href="mailto:hello@qapi.sh" class="package-cta secondary">
                                                    "Contact Sales"
                                                </A>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="package-footer">
                                        <div class="package-nav-bottom">
                                            <div class="bottom-nav-simple">
                                                {move || {
                                                    nav_info().map(|(prev_sku, _next_sku, _current_num, _total)| {
                                                        view! {
                                                            <A href={format!("/package/{}", prev_sku)} class="nav-link-simple">
                                                                "← Previous"
                                                            </A>
                                                        }.into_view()
                                                    }).unwrap_or_else(|| view! { <span></span> }.into_view())
                                                }}
                                                <A href="/" class="back-link-center">"Back to Packages"</A>
                                                {move || {
                                                    nav_info().map(|(_prev_sku, next_sku, _current_num, _total)| {
                                                        view! {
                                                            <A href={format!("/package/{}", next_sku)} class="nav-link-simple">
                                                                "Next →"
                                                            </A>
                                                        }.into_view()
                                                    }).unwrap_or_else(|| view! { <span></span> }.into_view())
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_view()
                        },
                        None => view! {
                            <div class="package-not-found">
                                <h1>"Package Not Found"</h1>
                                <p>"The requested package could not be found."</p>
                                <A href="/" class="cta-button primary">"Browse All Packages"</A>
                            </div>
                        }.into_view()
                    })
                }}
            </Suspense>
        </div>
    }
}

fn Dashboard() -> impl IntoView {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    struct Plan {
        cpu_cores: u16,
        ram_gb: u16,
        storage_gb: u32,
        gpu: String,
    }
    #[derive(Clone, Serialize, Deserialize)]
    struct CreateOrderRequest {
        plan: Plan,
        pq_enabled: bool,
        notes: Option<String>,
    }

    let (status, set_status) = create_signal(String::new());

    let order = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let body = CreateOrderRequest {
                plan: Plan {
                    cpu_cores: 16,
                    ram_gb: 128,
                    storage_gb: 2000,
                    gpu: "RTX_5090".into(),
                },
                pq_enabled: true,
                notes: Some("First order".into()),
            };
            let url = format!("{}/api/orders", api_base());
            let resp = gloo_net::http::Request::post(&url)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&body).unwrap())
                .unwrap()
                .send()
                .await;

            match resp {
                Ok(r) if r.status() == 200 => set_status.set("Order queued ✅".into()),
                Ok(r) => set_status.set(format!("Error {}", r.status())),
                Err(e) => set_status.set(format!("Network error: {e}")),
            }
        });
    };

    view! {
        <div style="max-width:900px;margin:2rem auto;padding:2rem;">
            <h2>"Your Servers"</h2>
            <button class="cta-button primary" on:click=order>"Order server"</button>
            <p>{move || status.get()}</p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Root default_theme=LeptonicTheme::Light>
            <Router>
                <main>
                    <Routes>
                        <Route path="/" view=Landing />
                        <Route path="/signup" view=Signup />
                        <Route path="/dashboard" view=Dashboard />
                        <Route path="/package/:sku" view=PackageDetail />
                    </Routes>
                </main>
            </Router>
        </Root>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

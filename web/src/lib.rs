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
struct ProvenanceOption {
    provenance_type: Provenance,
    quantity_available: u32,
    calculated_price: u32,
    discount_percentage: Option<f64>,
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
    provenances: Vec<ProvenanceOption>,
    min_price_usdc: Option<u32>,
    max_price_usdc: Option<u32>,
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
                                                            {if pkg.min_price_usdc.is_some() && pkg.max_price_usdc.is_some()
                                                                && pkg.min_price_usdc != pkg.max_price_usdc {
                                                                format!("${} ~ ${} USDC",
                                                                    pkg.min_price_usdc.unwrap_or(pkg.setup_price_usdc),
                                                                    pkg.max_price_usdc.unwrap_or(pkg.setup_price_usdc))
                                                            } else {
                                                                format!("${} USDC", pkg.setup_price_usdc)
                                                            }}
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
                                                            {if pkg.provenances.iter().any(|p| matches!(p.provenance_type, Provenance::Used { .. })) {
                                                                "♻️"
                                                            } else {
                                                                "🆕"
                                                            }}
                                                        </span>
                                                        <span>
                                                            {if pkg.provenances.len() > 1 {
                                                                format!("{} options", pkg.provenances.len())
                                                            } else if let Some(prov) = pkg.provenances.first() {
                                                                match &prov.provenance_type {
                                                                    Provenance::New => "New".to_string(),
                                                                    Provenance::Used { hours } => format!("Used ({}h)", hours),
                                                                }
                                                            } else {
                                                                "New".to_string()
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
                                Some((
                                    all_pkgs[prev_index].sku.clone(),
                                    all_pkgs[prev_index].name.clone(),
                                    all_pkgs[next_index].sku.clone(),
                                    all_pkgs[next_index].name.clone(),
                                    index + 1,
                                    total
                                ))
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
                                                nav_info().map(|(prev_sku, prev_name, next_sku, next_name, current_num, total)| {
                                                    view! {
                                                        <div class="package-nav-container">
                                                            <A href={format!("/package/{}", prev_sku)} class="nav-link-with-arrow nav-link-prev">
                                                                <span class="nav-arrow">"◀"</span>
                                                                <span class="nav-text">{prev_name}</span>
                                                            </A>
                                                            <div class="package-counter">
                                                                <span>{current_num}" of "{total}</span>
                                                            </div>
                                                            <A href={format!("/package/{}", next_sku)} class="nav-link-with-arrow nav-link-next">
                                                                <span class="nav-text">{next_name}</span>
                                                                <span class="nav-arrow">"▶"</span>
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
                                                // Use images from database
                                                pkg.images.iter().map(|img| {
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
                                                </div>
                                                <h3>"Provenance Options"</h3>
                                                <div class="provenance-options">
                                                    {pkg.provenances.iter().map(|prov| {
                                                        let provenance_label = match &prov.provenance_type {
                                                            Provenance::New => "Brand New".to_string(),
                                                            Provenance::Used { hours } => {
                                                                let years = *hours as f32 / 8760.0;
                                                                if years < 1.0 {
                                                                    format!("Used ({} hours)", hours)
                                                                } else {
                                                                    format!("Used ({:.1} years)", years)
                                                                }
                                                            }
                                                        };
                                                        view! {
                                                            <div class="provenance-option-card">
                                                                <div class="provenance-header">
                                                                    <span class="provenance-label">{provenance_label}</span>
                                                                    <span class="provenance-quantity">
                                                                        {format!("{} available", prov.quantity_available)}
                                                                    </span>
                                                                </div>
                                                                <div class="provenance-pricing">
                                                                    <span class="provenance-price">
                                                                        "$" {prov.calculated_price} " USDC"
                                                                    </span>
                                                                    {prov.discount_percentage.map(|discount| {
                                                                        view! {
                                                                            <span class="provenance-discount">
                                                                                {format!("-{:.0}%", discount)}
                                                                            </span>
                                                                        }
                                                                    })}
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>

                                            <div class="package-pricing">
                                                <h2>"Pricing"</h2>
                                                <div class="pricing-details">
                                                    <div class="price-item">
                                                        <span class="price-label">"Hardware & Setup"</span>
                                                        <span class="price-amount">
                                                            {if pkg.min_price_usdc.is_some() && pkg.max_price_usdc.is_some()
                                                                && pkg.min_price_usdc != pkg.max_price_usdc {
                                                                format!("${} ~ ${} USDC",
                                                                    pkg.min_price_usdc.unwrap_or(pkg.setup_price_usdc),
                                                                    pkg.max_price_usdc.unwrap_or(pkg.setup_price_usdc))
                                                            } else {
                                                                format!("${} USDC", pkg.setup_price_usdc)
                                                            }}
                                                        </span>
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
                                            <div class="package-nav-container">
                                                {move || {
                                                    nav_info().map(|(prev_sku, prev_name, next_sku, next_name, _current_num, _total)| {
                                                        view! {
                                                            <>
                                                                <A href={format!("/package/{}", prev_sku)} class="nav-link-with-arrow nav-link-prev">
                                                                    <span class="nav-arrow">"◀"</span>
                                                                    <span class="nav-text">{prev_name}</span>
                                                                </A>
                                                                <A href="/" class="back-link-center">"Back to Packages"</A>
                                                                <A href={format!("/package/{}", next_sku)} class="nav-link-with-arrow nav-link-next">
                                                                    <span class="nav-text">{next_name}</span>
                                                                    <span class="nav-arrow">"▶"</span>
                                                                </A>
                                                            </>
                                                        }.into_view()
                                                    }).unwrap_or_else(|| view! {
                                                        <A href="/" class="back-link-center back-link-solo">"Back to Packages"</A>
                                                    }.into_view())
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

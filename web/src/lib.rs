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
struct Package {
    id: String,
    name: String,
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
                    <div class="hero-badge">
                        "🔒 Post-Quantum Secured"
                    </div>
                    <h1 class="hero-title">
                        "Defensive AI Colocation"
                    </h1>
                    <p class="hero-subtitle">
                        "Deploy enterprise-grade AI infrastructure in the secure Balkan mountains. "
                        "Get dedicated hardware with tailored inference engines and dynamically loadable LLMs."
                    </p>
                    <div class="hero-features">
                        <div class="feature-item">
                            <span class="feature-icon">"⚡"</span>
                            <span>"Lightning Fast Inference"</span>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">"🛡️"</span>
                            <span>"Military-Grade Security"</span>
                        </div>
                        <div class="feature-item">
                            <span class="feature-icon">"🔧"</span>
                            <span>"Fully Managed Infrastructure"</span>
                        </div>
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
                    <h2 class="section-title">"Choose Your AI Powerhouse"</h2>
                    <p class="section-subtitle">
                        "All packages include dedicated hardware, custom inference engines, "
                        "and pre-configured LLMs optimized for your specific setup."
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

                                                <button class="cta-button" class:primary=is_popular>
                                                    "Deploy Now →"
                                                </button>
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
                    <h2>"Enterprise Security You Can Trust"</h2>
                    <div class="security-features">
                        <div class="security-item">
                            <div class="security-icon">"🔐"</div>
                            <h3>"Post-Quantum Cryptography"</h3>
                            <p>"Future-proof encryption that protects against quantum computing attacks"</p>
                        </div>
                        <div class="security-item">
                            <div class="security-icon">"🏔️"</div>
                            <h3>"Secure Datacenter"</h3>
                            <p>"Located in the geopolitically stable Balkan mountains with physical security"</p>
                        </div>
                        <div class="security-item">
                            <div class="security-icon">"🛡️"</div>
                            <h3>"Isolated Networks"</h3>
                            <p>"Each deployment runs in isolated environments with dedicated resources"</p>
                        </div>
                    </div>
                </div>
            </section>

            {/* CTA Section */}
            <section class="final-cta">
                <h2>"Ready to Deploy Your AI Infrastructure?"</h2>
                <p>"Join enterprises who trust their AI workloads to our secure, high-performance platform."</p>
                <A href="/signup" class="cta-button primary large">"Get Started Today →"</A>
            </section>

            {/* Footer */}
            <footer class="footer">
                <p>"© 2024 Defensive AI Colocation. Secured by post-quantum cryptography."</p>
            </footer>
        </div>

        <style>
            {r#"
            .landing-container {
                min-height: 100vh;
                background: linear-gradient(135deg, #0a0a0a 0%, #1a1a2e 100%);
                color: #ffffff;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
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

            .hero-content {
                flex: 1;
                max-width: 600px;
            }

            .hero-badge {
                display: inline-block;
                padding: 0.5rem 1rem;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                border-radius: 50px;
                font-size: 0.875rem;
                font-weight: 600;
                margin-bottom: 1.5rem;
            }

            .hero-title {
                font-size: 3.5rem;
                font-weight: 800;
                line-height: 1.1;
                margin-bottom: 1.5rem;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
            }

            .hero-subtitle {
                font-size: 1.25rem;
                line-height: 1.6;
                color: #b0b0b0;
                margin-bottom: 2rem;
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
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                border-radius: 8px;
                box-shadow: 0 8px 32px rgba(102, 126, 234, 0.3);
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

            .packages-section {
                padding: 6rem 2rem;
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
            }

            .section-subtitle {
                font-size: 1.125rem;
                color: #b0b0b0;
                max-width: 600px;
                margin: 0 auto;
                line-height: 1.6;
            }

            .packages-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
                gap: 2rem;
                margin-top: 3rem;
            }

            .package-card {
                background: linear-gradient(135deg, #1e1e2e 0%, #2a2a3e 100%);
                border: 1px solid #333;
                border-radius: 16px;
                padding: 2rem;
                position: relative;
                transition: all 0.3s ease;
                box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
            }

            .package-card:hover {
                transform: translateY(-4px);
                box-shadow: 0 12px 40px rgba(102, 126, 234, 0.2);
                border-color: #667eea;
            }

            .package-card.popular {
                border: 2px solid #667eea;
                transform: scale(1.05);
            }

            .popular-badge {
                position: absolute;
                top: -12px;
                left: 50%;
                transform: translateX(-50%);
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
                color: #667eea;
            }

            .package-description {
                color: #b0b0b0;
                line-height: 1.5;
            }

            .package-pricing {
                margin-bottom: 2rem;
                padding: 1.5rem;
                background: rgba(102, 126, 234, 0.1);
                border-radius: 12px;
                border: 1px solid rgba(102, 126, 234, 0.2);
            }

            .price-setup, .price-monthly {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 0.75rem;
            }

            .price-monthly {
                margin-bottom: 0;
                padding-top: 0.75rem;
                border-top: 1px solid rgba(102, 126, 234, 0.2);
            }

            .price-label {
                font-size: 0.875rem;
                color: #b0b0b0;
            }

            .price-amount {
                font-size: 1.25rem;
                font-weight: 700;
            }

            .price-amount.primary {
                color: #667eea;
                font-size: 1.5rem;
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
            }

            .spec-icon {
                font-size: 1.25rem;
                width: 24px;
                text-align: center;
            }

            .hardware-details, .package-includes {
                margin-bottom: 2rem;
            }

            .hardware-details h4, .package-includes h4 {
                font-size: 1rem;
                font-weight: 600;
                margin-bottom: 0.75rem;
                color: #667eea;
            }

            .hardware-details p {
                color: #b0b0b0;
                font-size: 0.9rem;
                line-height: 1.5;
            }

            .package-includes ul {
                list-style: none;
                padding: 0;
                margin: 0;
            }

            .package-includes li {
                font-size: 0.9rem;
                margin-bottom: 0.5rem;
                color: #b0b0b0;
            }

            .cta-button {
                width: 100%;
                padding: 1rem 2rem;
                background: linear-gradient(135deg, #333 0%, #555 100%);
                color: white;
                border: none;
                border-radius: 8px;
                font-size: 1rem;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.3s ease;
                text-decoration: none;
                display: inline-block;
                text-align: center;
            }

            .cta-button:hover {
                transform: translateY(-2px);
                box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
            }

            .cta-button.primary {
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            }

            .cta-button.large {
                padding: 1.25rem 3rem;
                font-size: 1.125rem;
            }

            .security-section {
                padding: 6rem 2rem;
                background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
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
                color: #667eea;
            }

            .security-item p {
                color: #b0b0b0;
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
                color: #b0b0b0;
                margin-bottom: 3rem;
                line-height: 1.6;
            }

            .footer {
                padding: 2rem;
                text-align: center;
                border-top: 1px solid #333;
                color: #666;
            }

            .loading-packages, .error-state {
                grid-column: 1 / -1;
                text-align: center;
                padding: 4rem 2rem;
            }

            .spinner {
                width: 40px;
                height: 40px;
                border: 4px solid #333;
                border-top: 4px solid #667eea;
                border-radius: 50%;
                animation: spin 1s linear infinite;
                margin: 0 auto 1rem;
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
        <div style="max-width:480px;margin:2rem auto;padding:2rem;background:#1e1e2e;border-radius:16px;">
            <h2>"Create your account"</h2>
            <input
                placeholder="Email"
                style="width:100%;padding:1rem;margin:1rem 0;border:1px solid #333;border-radius:8px;background:#2a2a3e;color:white;"
                on:input=move |e| set_email.set(event_target_value(&e))
            />
            <input
                type="password"
                placeholder="Password"
                style="width:100%;padding:1rem;margin:1rem 0;border:1px solid #333;border-radius:8px;background:#2a2a3e;color:white;"
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
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <main>
                    <Routes>
                        <Route path="/" view=Landing />
                        <Route path="/signup" view=Signup />
                        <Route path="/dashboard" view=Dashboard />
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

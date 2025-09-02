use leptonic::components::prelude::{LeptonicTheme, Root};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

fn api_base() -> String {
    if cfg!(debug_assertions) {
        "http://localhost:8081".to_string()
    } else {
        "".to_string() // Use relative URLs in production
    }
}

#[component]
fn Landing() -> impl IntoView {
    view! {
        <section style="max-width:900px;margin:3rem auto 2rem;">
            <h1>"Defensive AI Colocation"</h1>
            <p>
                "Managed AI servers in the Balkan mountains, secured with post-quantum crypto. "
                "Deploy inference platforms and LLMs tailored to your hardware."
            </p>
            <A href="/signup">"Get Started →"</A>
        </section>
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
        <div style="max-width:480px;margin:2rem auto;">
            <h2>"Create your account"</h2>
            <input placeholder="Email" on:input=move |e| set_email.set(event_target_value(&e))/>
            <input type="password" placeholder="Password" on:input=move |e| set_password.set(event_target_value(&e))/>
            <button on:click=submit>"Sign up"</button>
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
        <div style="max-width:900px;margin:2rem auto;">
            <h2>"Your Servers"</h2>
            <button on:click=order>"Order server"</button>
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

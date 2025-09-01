// src/bin/leptos_frontend.rs - Leptos frontend entry point

use leptos::*;
use fitness_advisor_ai::frontend::App;

fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| view! { <App/> })
}
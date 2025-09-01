// src/frontend/components/stats_cards.rs - User stats display

use leptos::prelude::*;

#[component]
pub fn StatsCards() -> impl IntoView {
    let (user_stats, _set_user_stats) = create_signal(UserStats::default());

    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
            <StatCard
                title="Body Mass Index"
                value=move || format!("{:.1}", user_stats.get().bmi)
                subtitle="Normal Range"
                gradient="from-purple-500 to-purple-700"
            />
            <StatCard
                title="Total Daily Energy"
                value=move || format!("{:,}", user_stats.get().tdee as u32)
                subtitle="kcal/day"
                gradient="from-blue-500 to-blue-700"
            />
            <StatCard
                title="Fitness Level"
                value=move || user_stats.get().fitness_level.clone()
                subtitle="8.5/10 Score"
                gradient="from-indigo-500 to-purple-600"
            />
        </div>
    }
}

#[component]
fn StatCard(
    title: &'static str,
    value: impl Fn() -> String + 'static,
    subtitle: &'static str,
    gradient: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("bg-gradient-to-br {} border-0 text-white relative overflow-hidden rounded-lg border border-white/10", gradient)>
            <div class="absolute inset-0 bg-black/10 backdrop-blur-sm"></div>
            <div class="p-6 relative z-10">
                <div class="flex items-center justify-between mb-4">
                    <div class="p-2 bg-white/20 rounded-lg backdrop-blur-sm">
                        <ActivityIcon/>
                    </div>
                </div>
                <div class="space-y-2">
                    <p class="text-white/80 text-sm">{title}</p>
                    <p class="text-2xl font-medium">{value}</p>
                    <p class="text-white/70 text-sm">{subtitle}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActivityIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
        </svg>
    }
}

#[derive(Clone, Debug)]
pub struct UserStats {
    pub bmi: f64,
    pub tdee: f64,
    pub fitness_level: String,
}

impl Default for UserStats {
    fn default() -> Self {
        Self {
            bmi: 22.4,
            tdee: 2340.0,
            fitness_level: "Advanced".to_string(),
        }
    }
}
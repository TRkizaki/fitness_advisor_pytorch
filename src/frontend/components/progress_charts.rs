// src/frontend/components/progress_charts.rs - Progress analytics charts

use leptos::prelude::*;

#[component]
pub fn ProgressCharts() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <ChartCard 
                title="Weight Progress"
                icon=<TrendingDownIcon/>
                value="-11 lbs this year"
                chart_color="purple"
            />
            <ChartCard 
                title="Calories Burned"
                icon=<FireIcon/>
                value="2,850 kcal this week"
                chart_color="blue"
            />
            <ChartCard 
                title="Strength Progress"
                icon=<TrendingUpIcon/>
                value="340 lb max"
                chart_color="green"
            />
        </div>
    }
}

#[component]
fn ChartCard(
    title: &'static str,
    icon: impl IntoView + 'static,
    value: &'static str,
    chart_color: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 text-white rounded-lg p-6">
            <div class="flex items-center gap-2 mb-4">
                {icon}
                <h3 class="text-lg">{title}</h3>
            </div>
            
            // Simulated chart area
            <div class="h-32 bg-gradient-to-t from-gray-800/50 to-transparent rounded-lg mb-4 relative overflow-hidden">
                <div class="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-purple-500/30 to-transparent rounded-lg"></div>
                <svg class="absolute inset-0 w-full h-full" viewBox="0 0 300 100">
                    <polyline
                        fill="none"
                        stroke=format!("rgb(var(--{}-500))", chart_color)
                        stroke-width="2"
                        points="0,80 50,60 100,70 150,45 200,35 250,25 300,20"
                    />
                    <For
                        each=|| (0..7).collect::<Vec<_>>()
                        key=|i| *i
                        children=move |i| {
                            let x = i * 50;
                            let y = 80 - (i * 10);
                            view! {
                                <circle cx=x cy=y r="3" class=format!("fill-{}-400", chart_color)/>
                            }
                        }
                    />
                </svg>
            </div>
            
            <p class="text-white/70 text-sm">{value}</p>
        </div>
    }
}

#[component]
fn TrendingDownIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0V9m0 8l-8-8-4 4-6-6"/>
        </svg>
    }
}

#[component]
fn FireIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.879 16.121A3 3 0 1012.015 11L11 14l4-4 3 3m-3-3l-3-3m3 3l3 3"/>
        </svg>
    }
}

#[component]
fn TrendingUpIcon() -> impl IntoView {
    view! {
        <svg class="h-5 w-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
        </svg>
    }
}
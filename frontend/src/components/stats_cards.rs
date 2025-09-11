use leptos::prelude::*;
use crate::components::icons::*;

#[derive(Clone)]
struct StatData {
    title: &'static str,
    value: &'static str,
    subtitle: &'static str,
    gradient: &'static str,
}

#[component]
fn StatCard(
    title: &'static str,
    value: &'static str,
    subtitle: &'static str,
    icon: impl IntoView,
    gradient: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("bg-gradient-to-br {} border-0 text-white relative overflow-hidden rounded-lg", gradient)>
            <div class="absolute inset-0 bg-black/10 backdrop-blur-sm"></div>
            <div class="p-6 relative z-10">
                <div class="flex items-center justify-between mb-4">
                    <div class="p-2 bg-white/20 rounded-lg backdrop-blur-sm">
                        {icon}
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
pub fn StatsCards() -> impl IntoView {
    let stats = vec![
        StatData {
            title: "Body Mass Index",
            value: "22.4",
            subtitle: "Normal Range",
            gradient: "from-purple-500 to-purple-700",
        },
        StatData {
            title: "Total Daily Energy",
            value: "2,340",
            subtitle: "kcal/day",
            gradient: "from-blue-500 to-blue-700",
        },
        StatData {
            title: "Fitness Level",
            value: "Advanced",
            subtitle: "8.5/10 Score",
            gradient: "from-indigo-500 to-purple-600",
        },
    ];

    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
            <StatCard
                title=stats[0].title
                value=stats[0].value
                subtitle=stats[0].subtitle
                icon=view! { <ActivityIcon/> }
                gradient=stats[0].gradient
            />
            <StatCard
                title=stats[1].title
                value=stats[1].value
                subtitle=stats[1].subtitle
                icon=view! { <TargetIcon/> }
                gradient=stats[1].gradient
            />
            <StatCard
                title=stats[2].title
                value=stats[2].value
                subtitle=stats[2].subtitle
                icon=view! { <TrendingUpIcon/> }
                gradient=stats[2].gradient
            />
        </div>
    }
}
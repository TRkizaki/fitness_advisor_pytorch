use leptos::prelude::*;
use crate::components::icons::*;

#[derive(Clone)]
struct WeightData {
    month: &'static str,
    weight: u32,
}

#[derive(Clone)]
struct WorkoutData {
    day: &'static str,
    calories: u32,
}

#[derive(Clone)]
struct StrengthData {
    month: &'static str,
    bench: u32,
    squat: u32,
    deadlift: u32,
}

#[component]
fn SimpleLineChart(data: Vec<(f64, f64)>, color: &'static str, width: u32, height: u32) -> impl IntoView {
    let max_x = data.iter().map(|(x, _)| *x).fold(0.0, f64::max);
    let max_y = data.iter().map(|(_, y)| *y).fold(0.0, f64::max);
    let min_y = data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    
    let scale_x = (width as f64 - 40.0) / max_x;
    let scale_y = (height as f64 - 40.0) / (max_y - min_y);
    
    let points: Vec<(f64, f64)> = data
        .iter()
        .map(|(x, y)| (20.0 + x * scale_x, height as f64 - 20.0 - (y - min_y) * scale_y))
        .collect();
    
    let path = points
        .iter()
        .enumerate()
        .map(|(i, (x, y))| {
            if i == 0 {
                format!("M {} {}", x, y)
            } else {
                format!("L {} {}", x, y)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    view! {
        <svg width=width height=height class="w-full h-full">
            <defs>
                <linearGradient id=format!("gradient-{}", color) x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stop-color=color stop-opacity="0.3"/>
                    <stop offset="95%" stop-color=color stop-opacity="0"/>
                </linearGradient>
            </defs>
            <path d=path fill="none" stroke=color stroke-width="2"/>
            {points.into_iter().map(|(x, y)| {
                view! {
                    <circle cx=x cy=y r="3" fill=color/>
                }
            }).collect::<Vec<_>>()}
        </svg>
    }
}

#[component]
pub fn ProgressCharts() -> impl IntoView {
    let weight_data = vec![
        WeightData { month: "Jan", weight: 180 },
        WeightData { month: "Feb", weight: 178 },
        WeightData { month: "Mar", weight: 175 },
        WeightData { month: "Apr", weight: 173 },
        WeightData { month: "May", weight: 171 },
        WeightData { month: "Jun", weight: 169 },
    ];

    let workout_data = vec![
        WorkoutData { day: "Mon", calories: 420 },
        WorkoutData { day: "Tue", calories: 380 },
        WorkoutData { day: "Wed", calories: 450 },
        WorkoutData { day: "Thu", calories: 320 },
        WorkoutData { day: "Fri", calories: 490 },
        WorkoutData { day: "Sat", calories: 380 },
        WorkoutData { day: "Sun", calories: 410 },
    ];

    let strength_data = vec![
        StrengthData { month: "Jan", bench: 185, squat: 225, deadlift: 275 },
        StrengthData { month: "Feb", bench: 190, squat: 235, deadlift: 285 },
        StrengthData { month: "Mar", bench: 195, squat: 245, deadlift: 295 },
        StrengthData { month: "Apr", bench: 200, squat: 255, deadlift: 305 },
        StrengthData { month: "May", bench: 205, squat: 265, deadlift: 315 },
        StrengthData { month: "Jun", bench: 210, squat: 275, deadlift: 325 },
    ];

    let weight_chart_data: Vec<(f64, f64)> = weight_data
        .iter()
        .enumerate()
        .map(|(i, w)| (i as f64, w.weight as f64))
        .collect();

    let workout_chart_data: Vec<(f64, f64)> = workout_data
        .iter()
        .enumerate()
        .map(|(i, w)| (i as f64, w.calories as f64))
        .collect();

    let bench_chart_data: Vec<(f64, f64)> = strength_data
        .iter()
        .enumerate()
        .map(|(i, s)| (i as f64, s.bench as f64))
        .collect();

    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
            // Weight Progress
            <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
                <div class="p-6 border-b border-white/10">
                    <h3 class="flex items-center gap-2 text-lg font-semibold">
                        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3"></path>
                        </svg>
                        "Weight Progress"
                    </h3>
                </div>
                <div class="p-6">
                    <div class="h-48 mb-4">
                        <SimpleLineChart data=weight_chart_data color="#8b5cf6" width=300 height=200/>
                    </div>
                    <div class="flex items-center gap-2 text-green-400">
                        <TrendingUpIcon/>
                        <span class="text-sm">"-11 lbs this year"</span>
                    </div>
                </div>
            </div>

            // Weekly Calories Burned
            <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
                <div class="p-6 border-b border-white/10">
                    <h3 class="flex items-center gap-2 text-lg font-semibold">
                        <ZapIcon/>
                        "Calories Burned"
                    </h3>
                </div>
                <div class="p-6">
                    <div class="h-48 mb-4">
                        <SimpleLineChart data=workout_chart_data color="#3b82f6" width=300 height=200/>
                    </div>
                    <div class="flex items-center gap-2 text-blue-400">
                        <TrendingUpIcon/>
                        <span class="text-sm">"2,850 kcal this week"</span>
                    </div>
                </div>
            </div>

            // Strength Progress
            <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white lg:col-span-2 xl:col-span-1">
                <div class="p-6 border-b border-white/10">
                    <h3 class="flex items-center gap-2 text-lg font-semibold">
                        <TrendingUpIcon/>
                        "Strength Progress"
                    </h3>
                </div>
                <div class="p-6">
                    <div class="h-48 mb-4">
                        <SimpleLineChart data=bench_chart_data color="#8b5cf6" width=300 height=200/>
                    </div>
                    <div class="grid grid-cols-3 gap-2 text-sm">
                        <div class="flex items-center gap-1">
                            <div class="w-3 h-3 bg-purple-500 rounded"></div>
                            <span class="text-white/80">"Bench"</span>
                        </div>
                        <div class="flex items-center gap-1">
                            <div class="w-3 h-3 bg-blue-500 rounded"></div>
                            <span class="text-white/80">"Squat"</span>
                        </div>
                        <div class="flex items-center gap-1">
                            <div class="w-3 h-3 bg-green-500 rounded"></div>
                            <span class="text-white/80">"Deadlift"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
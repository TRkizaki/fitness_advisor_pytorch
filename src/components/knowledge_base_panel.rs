use leptos::prelude::*;
use crate::api::rag_client::{
    RagApiClient, SemanticSearchRequest, SemanticSearchResult, DocumentType, 
    RecommendationRequest, SmartRecommendation, RecommendationType, UserContext,
    KnowledgeInsight, AnalyticsData, RagApiError
};
use crate::components::semantic_search::{SemanticSearchBar, SearchFilters, SearchResults};
use crate::components::document_manager::DocumentManager;
use serde_json::{Value, json};

#[component]
pub fn KnowledgeBasePanel() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("search".to_string());
    let (search_query, set_search_query) = signal(String::new());
    let (selected_types, set_selected_types) = signal(Vec::<DocumentType>::new());
    let (search_results, set_search_results) = signal(Vec::<SemanticSearchResult>::new());
    let (search_loading, set_search_loading) = signal(false);
    let (search_error, set_search_error) = signal(String::new());
    let (recommendations, set_recommendations) = signal(Vec::<SmartRecommendation>::new());
    let (rec_loading, set_rec_loading) = signal(false);
    let (insights, set_insights) = signal(Vec::<KnowledgeInsight>::new());
    let (analytics, set_analytics) = signal(None::<AnalyticsData>());

    let perform_search = move |query: String| {
        set_search_query.set(query.clone());
        set_search_loading.set(true);
        set_search_error.set(String::new());

        spawn_local(async move {
            let request = SemanticSearchRequest {
                query,
                document_types: if selected_types.get().is_empty() { 
                    None 
                } else { 
                    Some(selected_types.get()) 
                },
                limit: Some(10),
                similarity_threshold: Some(0.3),
            };

            match RagApiClient::semantic_search(request).await {
                Ok(results) => {
                    set_search_results.set(results);
                }
                Err(e) => {
                    set_search_error.set(format!("Search failed: {}", e));
                }
            }
            set_search_loading.set(false);
        });
    };

    let load_recommendations = move |rec_type: RecommendationType| {
        set_rec_loading.set(true);
        spawn_local(async move {
            let request = RecommendationRequest {
                user_context: UserContext {
                    user_id: "demo-user".to_string(),
                    fitness_goals: vec!["muscle_gain".to_string(), "strength".to_string()],
                    current_stats: json!({
                        "weight": 75,
                        "height": 180,
                        "body_fat": 15
                    }),
                    preferences: vec!["high_protein".to_string(), "compound_exercises".to_string()],
                    workout_history: None,
                },
                recommendation_type: rec_type,
                preferences: None,
                limit: Some(5),
            };

            match RagApiClient::get_smart_recommendations(request).await {
                Ok(recs) => {
                    set_recommendations.set(recs);
                }
                Err(_e) => {
                    // Fallback to sample recommendations on error
                    let sample_recs = create_sample_recommendations(rec_type);
                    set_recommendations.set(sample_recs);
                }
            }
            set_rec_loading.set(false);
        });
    };

    let load_insights = move |_| {
        spawn_local(async move {
            match RagApiClient::get_knowledge_insights(None).await {
                Ok(insights_data) => {
                    set_insights.set(insights_data);
                }
                Err(_e) => {
                    // Fallback to sample insights
                    let sample_insights = create_sample_insights();
                    set_insights.set(sample_insights);
                }
            }
        });
    };

    let load_analytics = move |_| {
        spawn_local(async move {
            match RagApiClient::get_analytics().await {
                Ok(analytics_data) => {
                    set_analytics.set(Some(analytics_data));
                }
                Err(_e) => {
                    // Fallback to sample analytics
                    let sample_analytics = create_sample_analytics();
                    set_analytics.set(Some(sample_analytics));
                }
            }
        });
    };

    // Load initial data
    Effect::new(move |_| {
        load_insights(());
        load_analytics(());
        load_recommendations(RecommendationType::WorkoutPlan);
    });

    view! {
        <div class="bg-black/40 backdrop-blur-lg border border-white/10 rounded-lg text-white">
            <div class="p-6 border-b border-white/10">
                <h3 class="flex items-center gap-2 text-lg font-semibold">
                    "🧠 AI Knowledge Base"
                </h3>
            </div>
            
            <div class="p-6 space-y-6">
                // Tab navigation
                <div class="flex space-x-1 bg-white/5 rounded-lg p-1">
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}", 
                            if active_tab.get() == "search" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("search".to_string())
                    >
                        "🔍 Search"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "recommendations" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("recommendations".to_string())
                    >
                        "💡 Recommendations"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "browse" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("browse".to_string())
                    >
                        "📚 Browse"
                    </button>
                    <button 
                        class=move || format!("px-4 py-2 rounded-md text-sm transition-all {}",
                            if active_tab.get() == "analytics" { "bg-purple-600 text-white" } else { "text-white/70 hover:text-white hover:bg-white/10" })
                        on:click=move |_| set_active_tab.set("analytics".to_string())
                    >
                        "📊 Analytics"
                    </button>
                </div>

                // Tab Content
                {move || {
                    match active_tab.get().as_str() {
                        "search" => view! {
                            <div class="space-y-6">
                                // Search interface
                                <div class="space-y-4">
                                    <SemanticSearchBar 
                                        query=search_query.into()
                                        on_search=Callback::new(perform_search)
                                    />
                                    
                                    <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                                        <div class="lg:col-span-1">
                                            <SearchFilters
                                                selected_types=selected_types.into()
                                                on_filter_change=Callback::new(set_selected_types)
                                            />
                                        </div>
                                        <div class="lg:col-span-3">
                                            <SearchResults
                                                results=search_results.into()
                                                loading=search_loading.into()
                                                error=search_error.into()
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into(),
                        
                        "recommendations" => view! {
                            <div class="space-y-6">
                                <div class="flex justify-between items-center">
                                    <h4 class="text-lg font-medium">"AI-Powered Recommendations"</h4>
                                    {move || {
                                        if rec_loading.get() {
                                            view! {
                                                <div class="flex items-center gap-2 text-white/70">
                                                    <div class="animate-spin w-4 h-4 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                                    "Loading..."
                                                </div>
                                            }.into()
                                        } else {
                                            view! { <div></div> }.into()
                                        }
                                    }}
                                </div>
                                
                                // Recommendation categories
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                    {[
                                        (RecommendationType::WorkoutPlan, "🏋️ Workout Plans", "Get personalized workout recommendations"),
                                        (RecommendationType::NutritionAdvice, "🥗 Nutrition", "Discover optimal nutrition strategies"),
                                        (RecommendationType::RecoveryTips, "😴 Recovery", "Learn effective recovery techniques")
                                    ].iter().map(|(rec_type, title, desc)| {
                                        let rec_type_clone = rec_type.clone();
                                        view! {
                                            <button
                                                class="bg-white/5 rounded-lg p-4 border border-white/10 hover:border-purple-500/50 transition-colors text-left"
                                                on:click=move |_| load_recommendations(rec_type_clone.clone())
                                            >
                                                <h5 class="font-medium mb-2">{title}</h5>
                                                <p class="text-white/70 text-sm">{desc}</p>
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                
                                // Recommendations display
                                <div class="space-y-4">
                                    {move || {
                                        let recs = recommendations.get();
                                        if recs.is_empty() {
                                            view! {
                                                <div class="text-center py-8 bg-white/5 rounded-lg border border-dashed border-white/20">
                                                    <p class="text-white/60">"Select a category above to view recommendations"</p>
                                                </div>
                                            }.into()
                                        } else {
                                            view! {
                                                <div class="space-y-3">
                                                    {recs.into_iter().map(|rec| {
                                                        view! {
                                                            <RecommendationCard recommendation=rec/>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }.into()
                                        }
                                    }}
                                </div>
                            </div>
                        }.into(),
                        
                        "browse" => view! {
                            <div class="space-y-6">
                                <h4 class="text-lg font-medium">"Document Manager"</h4>
                                <DocumentManager/>
                            </div>
                        }.into(),
                        
                        _ => view! {
                            <div class="space-y-6">
                                <h4 class="text-lg font-medium">"Knowledge Base Analytics"</h4>
                                
                                {move || {
                                    if let Some(analytics_data) = analytics.get() {
                                        view! {
                                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                                <div class="bg-white/5 rounded-lg p-4">
                                                    <h5 class="font-medium mb-2">"Total Documents"</h5>
                                                    <p class="text-2xl font-bold text-purple-400">{analytics_data.total_documents}</p>
                                                </div>
                                                <div class="bg-white/5 rounded-lg p-4">
                                                    <h5 class="font-medium mb-2">"Total Searches"</h5>
                                                    <p class="text-2xl font-bold text-blue-400">{analytics_data.search_metrics.total_searches}</p>
                                                </div>
                                                <div class="bg-white/5 rounded-lg p-4">
                                                    <h5 class="font-medium mb-2">"Avg Results"</h5>
                                                    <p class="text-2xl font-bold text-green-400">{format!("{:.1}", analytics_data.search_metrics.avg_results_per_search)}</p>
                                                </div>
                                            </div>
                                        }.into()
                                    } else {
                                        view! {
                                            <div class="flex items-center justify-center py-12">
                                                <div class="animate-spin w-6 h-6 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                            </div>
                                        }.into()
                                    }
                                }}
                                
                                // Knowledge insights
                                <div class="space-y-4">
                                    <h5 class="font-medium">"Knowledge Insights"</h5>
                                    <div class="space-y-3">
                                        {move || {
                                            insights.get().into_iter().map(|insight| {
                                                view! {
                                                    <div class="bg-white/5 rounded-lg p-4 border border-white/10">
                                                        <div class="flex justify-between items-start mb-2">
                                                            <h6 class="font-medium text-purple-400">{insight.category}</h6>
                                                            <span class="text-xs text-white/60">{format!("{:.1}% confidence", insight.confidence * 100.0)}</span>
                                                        </div>
                                                        <p class="text-white/80 text-sm">{insight.insight}</p>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()
                                        }}
                                    </div>
                                </div>
                            </div>
                        }.into()
                    }
                }}
            </div>
        </div>
    }
}

#[component] 
fn RecommendationCard(recommendation: SmartRecommendation) -> impl IntoView {
    let rec_type_info = match recommendation.recommendation_type {
        RecommendationType::WorkoutPlan => ("🏋️", "Workout Plan"),
        RecommendationType::NutritionAdvice => ("🥗", "Nutrition"),
        RecommendationType::ExerciseForm => ("💪", "Exercise Form"),
        RecommendationType::RecoveryTips => ("😴", "Recovery"),
        RecommendationType::ProgressOptimization => ("📊", "Progress"),
    };

    view! {
        <div class="bg-gradient-to-r from-purple-500/20 to-blue-500/20 rounded-lg border border-white/10 p-4">
            <div class="flex items-start justify-between mb-3">
                <div class="flex items-center gap-2">
                    <span>{rec_type_info.0}</span>
                    <span class="text-xs bg-white/20 px-2 py-1 rounded">{rec_type_info.1}</span>
                </div>
                <span class="text-xs text-white/60">
                    {format!("{:.1}% relevance", recommendation.relevance_score * 100.0)}
                </span>
            </div>
            
            <h5 class="font-medium text-white mb-2">{recommendation.title}</h5>
            <p class="text-white/80 text-sm mb-3">{recommendation.description}</p>
            
            {if !recommendation.action_items.is_empty() {
                view! {
                    <div class="space-y-1">
                        <h6 class="text-xs font-medium text-white/70">"Action Items:"</h6>
                        <ul class="space-y-1">
                            {recommendation.action_items.into_iter().map(|item| {
                                view! { <li class="text-xs text-white/60">"• " {item}</li> }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                }.into()
            } else {
                view! { <div></div> }.into()
            }}
        </div>
    }
}

// Helper functions for sample data
fn create_sample_recommendations(rec_type: RecommendationType) -> Vec<SmartRecommendation> {
    match rec_type {
        RecommendationType::WorkoutPlan => vec![
            SmartRecommendation {
                id: "1".to_string(),
                title: "Progressive Overload Strength Program".to_string(),
                description: "4-week program focusing on compound movements with systematic weight increases".to_string(),
                recommendation_type: RecommendationType::WorkoutPlan,
                relevance_score: 0.92,
                supporting_documents: vec!["strength_guide_1".to_string()],
                action_items: vec![
                    "Start with 3 sets of 5 reps".to_string(),
                    "Increase weight by 5lbs weekly".to_string(),
                ],
                metadata: json!({}),
            }
        ],
        RecommendationType::NutritionAdvice => vec![
            SmartRecommendation {
                id: "2".to_string(),
                title: "High Protein Muscle Building Diet".to_string(),
                description: "Optimized macronutrient distribution for lean muscle gain".to_string(),
                recommendation_type: RecommendationType::NutritionAdvice,
                relevance_score: 0.88,
                supporting_documents: vec!["nutrition_guide_1".to_string()],
                action_items: vec![
                    "Target 1.6g protein per kg body weight".to_string(),
                    "Eat protein within 2h post-workout".to_string(),
                ],
                metadata: json!({}),
            }
        ],
        _ => vec![]
    }
}

fn create_sample_insights() -> Vec<KnowledgeInsight> {
    vec![
        KnowledgeInsight {
            category: "Exercise Effectiveness".to_string(),
            insight: "Compound exercises show 40% better strength gains than isolation exercises".to_string(),
            confidence: 0.89,
            supporting_evidence: vec!["research_paper_1".to_string(), "study_analysis_2".to_string()],
        }
    ]
}

fn create_sample_analytics() -> AnalyticsData {
    AnalyticsData {
        total_documents: 247,
        documents_by_type: json!({
            "fitness_guide": 89,
            "nutrition_info": 76,
            "exercise_description": 45,
            "research_paper": 23,
            "user_manual": 12,
            "faq": 2
        }),
        search_metrics: crate::api::rag_client::SearchMetrics {
            total_searches: 1428,
            avg_results_per_search: 8.4,
            top_queries: vec![],
        },
        popular_topics: vec![],
        knowledge_gaps: vec!["Advanced recovery techniques".to_string()],
    }
}
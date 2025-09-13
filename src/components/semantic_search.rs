use leptos::prelude::*;
use crate::api::rag_client::{RagApiClient, SemanticSearchRequest, SemanticSearchResult, DocumentType, RagApiError};

#[component]
pub fn SemanticSearchBar(
    #[prop(into)] query: Signal<String>,
    #[prop(into)] on_search: Callback<String>,
) -> impl IntoView {
    let (local_query, set_local_query) = signal(String::new());

    Effect::new(move |_| {
        set_local_query.set(query.get());
    });

    let handle_search = move |_| {
        let current_query = local_query.get();
        if !current_query.trim().is_empty() {
            on_search.call(current_query);
        }
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            handle_search(());
        }
    };

    view! {
        <div class="relative">
            <div class="flex gap-2">
                <div class="flex-1 relative">
                    <input
                        type="text"
                        placeholder="Search fitness knowledge base..."
                        class="w-full px-4 py-3 pl-10 bg-white/10 border border-white/20 rounded-lg text-white placeholder-white/50 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500"
                        prop:value=move || local_query.get()
                        on:input=move |ev| set_local_query.set(event_target_value(&ev))
                        on:keydown=handle_keydown
                    />
                    <div class="absolute left-3 top-1/2 transform -translate-y-1/2 text-white/40">
                        "🔍"
                    </div>
                </div>
                <button
                    on:click=handle_search
                    class="px-6 py-3 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white rounded-lg transition-all"
                >
                    "Search"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn SearchFilters(
    #[prop(into)] selected_types: Signal<Vec<DocumentType>>,
    #[prop(into)] on_filter_change: Callback<Vec<DocumentType>>,
) -> impl IntoView {
    let document_types = vec![
        (DocumentType::FitnessGuide, "Fitness Guides", "🏋️"),
        (DocumentType::NutritionInfo, "Nutrition Info", "🍎"),
        (DocumentType::ExerciseDescription, "Exercises", "💪"),
        (DocumentType::ResearchPaper, "Research", "📄"),
        (DocumentType::UserManual, "Manuals", "📖"),
        (DocumentType::FAQ, "FAQ", "❓"),
    ];

    let toggle_filter = move |doc_type: DocumentType| {
        let mut current_types = selected_types.get();
        if let Some(pos) = current_types.iter().position(|t| matches!((t, &doc_type), 
            (DocumentType::FitnessGuide, DocumentType::FitnessGuide) |
            (DocumentType::NutritionInfo, DocumentType::NutritionInfo) |
            (DocumentType::ExerciseDescription, DocumentType::ExerciseDescription) |
            (DocumentType::ResearchPaper, DocumentType::ResearchPaper) |
            (DocumentType::UserManual, DocumentType::UserManual) |
            (DocumentType::FAQ, DocumentType::FAQ)
        )) {
            current_types.remove(pos);
        } else {
            current_types.push(doc_type);
        }
        on_filter_change.call(current_types);
    };

    view! {
        <div class="space-y-3">
            <h4 class="text-sm font-medium text-white/80">"Filter by Type"</h4>
            <div class="flex flex-wrap gap-2">
                {document_types.into_iter().map(|(doc_type, label, icon)| {
                    let doc_type_clone = doc_type.clone();
                    let is_selected = move || {
                        selected_types.get().iter().any(|t| matches!((t, &doc_type_clone),
                            (DocumentType::FitnessGuide, DocumentType::FitnessGuide) |
                            (DocumentType::NutritionInfo, DocumentType::NutritionInfo) |
                            (DocumentType::ExerciseDescription, DocumentType::ExerciseDescription) |
                            (DocumentType::ResearchPaper, DocumentType::ResearchPaper) |
                            (DocumentType::UserManual, DocumentType::UserManual) |
                            (DocumentType::FAQ, DocumentType::FAQ)
                        ))
                    };
                    
                    view! {
                        <button
                            on:click=move |_| toggle_filter(doc_type_clone.clone())
                            class=move || format!(
                                "px-3 py-2 rounded-lg text-sm transition-all flex items-center gap-2 {}",
                                if is_selected() {
                                    "bg-purple-600 text-white border border-purple-500"
                                } else {
                                    "bg-white/5 text-white/70 border border-white/20 hover:bg-white/10 hover:border-white/30"
                                }
                            )
                        >
                            <span>{icon}</span>
                            <span>{label}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn SearchResultCard(result: SemanticSearchResult) -> impl IntoView {
    let document_type_info = match result.document_type {
        DocumentType::FitnessGuide => ("🏋️", "Fitness Guide", "bg-green-500/20 border-green-500/30"),
        DocumentType::NutritionInfo => ("🍎", "Nutrition", "bg-orange-500/20 border-orange-500/30"),
        DocumentType::ExerciseDescription => ("💪", "Exercise", "bg-blue-500/20 border-blue-500/30"),
        DocumentType::ResearchPaper => ("📄", "Research", "bg-purple-500/20 border-purple-500/30"),
        DocumentType::UserManual => ("📖", "Manual", "bg-gray-500/20 border-gray-500/30"),
        DocumentType::FAQ => ("❓", "FAQ", "bg-yellow-500/20 border-yellow-500/30"),
    };

    let relevance_color = if result.similarity_score >= 0.8 {
        "text-green-400"
    } else if result.similarity_score >= 0.6 {
        "text-yellow-400"
    } else {
        "text-red-400"
    };

    view! {
        <div class="bg-white/5 border border-white/10 rounded-lg p-4 hover:border-white/20 transition-all">
            <div class="flex items-start justify-between mb-3">
                <div class="flex items-center gap-2">
                    <span class=format!("px-2 py-1 rounded text-xs {}", document_type_info.2)>
                        {document_type_info.0} " " {document_type_info.1}
                    </span>
                </div>
                <div class="flex items-center gap-1">
                    <span class="text-white/60 text-xs">"Relevance:"</span>
                    <span class=format!("text-xs font-medium {}", relevance_color)>
                        {format!("{:.1}%", result.similarity_score * 100.0)}
                    </span>
                </div>
            </div>
            
            <h4 class="text-white font-medium mb-2">{result.title}</h4>
            
            <p class="text-white/70 text-sm leading-relaxed mb-3">
                {if result.content.len() > 200 {
                    format!("{}...", &result.content[..200])
                } else {
                    result.content
                }}
            </p>
            
            <div class="flex items-center justify-between text-xs text-white/50">
                <span>
                    "Updated: " {result.updated_at.split('T').next().unwrap_or(&result.updated_at)}
                </span>
                <button class="text-purple-400 hover:text-purple-300 transition-colors">
                    "View Details →"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn SearchResults(
    #[prop(into)] results: Signal<Vec<SemanticSearchResult>>,
    #[prop(into)] loading: Signal<bool>,
    #[prop(into)] error: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            {move || {
                if loading.get() {
                    view! {
                        <div class="flex items-center justify-center py-12">
                            <div class="flex items-center gap-3 text-white/70">
                                <div class="animate-spin w-5 h-5 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                <span>"Searching knowledge base..."</span>
                            </div>
                        </div>
                    }.into()
                } else if !error.get().is_empty() {
                    view! {
                        <div class="bg-red-600/20 border border-red-500/30 rounded-lg p-4">
                            <h4 class="text-red-300 font-medium mb-2">"Search Error"</h4>
                            <p class="text-red-200 text-sm">{error.get()}</p>
                        </div>
                    }.into()
                } else {
                    let results_list = results.get();
                    if results_list.is_empty() {
                        view! {
                            <div class="text-center py-12 bg-white/5 rounded-lg border border-dashed border-white/20">
                                <div class="text-4xl text-white/40 mb-3">"🔍"</div>
                                <h4 class="text-white/60 font-medium mb-2">"No results found"</h4>
                                <p class="text-white/40 text-sm">"Try different keywords or adjust your filters"</p>
                            </div>
                        }.into()
                    } else {
                        view! {
                            <div class="space-y-4">
                                <div class="flex items-center justify-between">
                                    <h4 class="text-white/80 text-sm">
                                        "Found " {results_list.len()} " result"
                                        {if results_list.len() != 1 { "s" } else { "" }}
                                    </h4>
                                    <button class="text-purple-400 hover:text-purple-300 text-sm transition-colors">
                                        "Sort by relevance ↓"
                                    </button>
                                </div>
                                
                                <div class="space-y-3">
                                    {results_list.into_iter().map(|result| {
                                        view! { <SearchResultCard result=result/> }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into()
                    }
                }
            }}
        </div>
    }
}
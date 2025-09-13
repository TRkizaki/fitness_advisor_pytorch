use leptos::prelude::*;
use crate::api::rag_client::{
    RagApiClient, DocumentUploadRequest, Document, DocumentType, EmbeddingStatus, RagApiError
};
use serde_json::{Value, json};

#[component]
pub fn DocumentManager() -> impl IntoView {
    let (documents, set_documents) = signal(Vec::<Document>::new());
    let (selected_documents, set_selected_documents) = signal(Vec::<String>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(String::new());
    let (show_upload_form, set_show_upload_form) = signal(false);
    let (filter_type, set_filter_type) = signal(None::<DocumentType>);

    // Load documents on component mount
    Effect::new(move |_| {
        load_documents(filter_type.get(), set_documents, set_loading, set_error);
    });

    // Reload when filter changes
    Effect::new(move |_| {
        let _ = filter_type.get(); // Track changes
        load_documents(filter_type.get(), set_documents, set_loading, set_error);
    });

    let toggle_document_selection = move |doc_id: String| {
        set_selected_documents.update(|selected| {
            if let Some(pos) = selected.iter().position(|id| id == &doc_id) {
                selected.remove(pos);
            } else {
                selected.push(doc_id);
            }
        });
    };

    let delete_selected = move |_| {
        let selected = selected_documents.get();
        if selected.is_empty() {
            return;
        }

        set_loading.set(true);
        spawn_local(async move {
            match RagApiClient::bulk_delete_documents(selected.clone()).await {
                Ok(_) => {
                    set_selected_documents.set(Vec::new());
                    load_documents(filter_type.get(), set_documents, set_loading, set_error);
                }
                Err(e) => {
                    set_error.set(format!("Failed to delete documents: {}", e));
                    set_loading.set(false);
                }
            }
        });
    };

    let refresh_documents = move |_| {
        load_documents(filter_type.get(), set_documents, set_loading, set_error);
    };

    view! {
        <div class="space-y-6">
            // Header with controls
            <div class="flex justify-between items-center">
                <div class="flex items-center gap-4">
                    <h4 class="text-lg font-medium">"Document Library"</h4>
                    <select
                        class="bg-white/10 border border-white/20 rounded px-3 py-1 text-sm text-white"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if value == "all" {
                                set_filter_type.set(None);
                            } else {
                                match value.as_str() {
                                    "fitness_guide" => set_filter_type.set(Some(DocumentType::FitnessGuide)),
                                    "nutrition_info" => set_filter_type.set(Some(DocumentType::NutritionInfo)),
                                    "exercise_description" => set_filter_type.set(Some(DocumentType::ExerciseDescription)),
                                    "research_paper" => set_filter_type.set(Some(DocumentType::ResearchPaper)),
                                    "user_manual" => set_filter_type.set(Some(DocumentType::UserManual)),
                                    "faq" => set_filter_type.set(Some(DocumentType::FAQ)),
                                    _ => set_filter_type.set(None),
                                }
                            }
                        }
                    >
                        <option value="all">"All Types"</option>
                        <option value="fitness_guide">"🏋️ Fitness Guides"</option>
                        <option value="nutrition_info">"🍎 Nutrition Info"</option>
                        <option value="exercise_description">"💪 Exercises"</option>
                        <option value="research_paper">"📄 Research"</option>
                        <option value="user_manual">"📖 Manuals"</option>
                        <option value="faq">"❓ FAQ"</option>
                    </select>
                </div>

                <div class="flex gap-2">
                    <button
                        on:click=refresh_documents
                        class="px-3 py-1 bg-white/10 hover:bg-white/20 border border-white/20 rounded text-sm transition-colors"
                        disabled=loading
                    >
                        "🔄 Refresh"
                    </button>
                    <button
                        on:click=move |_| set_show_upload_form.update(|show| *show = !*show)
                        class="px-3 py-1 bg-purple-600 hover:bg-purple-700 rounded text-sm transition-colors"
                    >
                        "📁 Upload"
                    </button>
                </div>
            </div>

            // Error display
            {move || {
                let error_msg = error.get();
                if !error_msg.is_empty() {
                    view! {
                        <div class="bg-red-600/20 border border-red-500/30 rounded-lg p-3">
                            <p class="text-red-300">{error_msg}</p>
                        </div>
                    }.into()
                } else {
                    view! { <div></div> }.into()
                }
            }}

            // Upload form
            {move || {
                if show_upload_form.get() {
                    view! {
                        <DocumentUploadForm
                            on_upload_success=Callback::new(move |_| {
                                set_show_upload_form.set(false);
                                load_documents(filter_type.get(), set_documents, set_loading, set_error);
                            })
                            on_cancel=Callback::new(move |_| set_show_upload_form.set(false))
                        />
                    }.into()
                } else {
                    view! { <div></div> }.into()
                }
            }}

            // Bulk operations
            {move || {
                let selected_count = selected_documents.get().len();
                if selected_count > 0 {
                    view! {
                        <div class="flex items-center gap-4 p-3 bg-yellow-500/20 border border-yellow-500/30 rounded-lg">
                            <span class="text-yellow-300 text-sm">
                                {selected_count} " document" {if selected_count != 1 { "s" } else { "" }} " selected"
                            </span>
                            <BulkOperations 
                                selected_count=selected_count
                                on_delete=Callback::new(delete_selected)
                                on_clear=Callback::new(move |_| set_selected_documents.set(Vec::new()))
                            />
                        </div>
                    }.into()
                } else {
                    view! { <div></div> }.into()
                }
            }}

            // Documents list
            {move || {
                if loading.get() {
                    view! {
                        <div class="flex items-center justify-center py-12">
                            <div class="flex items-center gap-3 text-white/70">
                                <div class="animate-spin w-5 h-5 border-2 border-purple-500 border-t-transparent rounded-full"></div>
                                <span>"Loading documents..."</span>
                            </div>
                        </div>
                    }.into()
                } else {
                    let docs = documents.get();
                    if docs.is_empty() {
                        view! {
                            <div class="text-center py-12 bg-white/5 rounded-lg border border-dashed border-white/20">
                                <div class="text-4xl text-white/40 mb-3">"📄"</div>
                                <h4 class="text-white/60 font-medium mb-2">"No documents found"</h4>
                                <p class="text-white/40 text-sm">"Upload some documents to get started"</p>
                            </div>
                        }.into()
                    } else {
                        view! {
                            <DocumentList
                                documents=docs
                                selected_documents=selected_documents.into()
                                on_document_select=Callback::new(toggle_document_selection)
                            />
                        }.into()
                    }
                }
            }}
        </div>
    }
}

#[component]
fn DocumentUploadForm(
    on_upload_success: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (doc_type, set_doc_type) = signal(DocumentType::FitnessGuide);
    let (tags, set_tags) = signal(String::new());
    let (uploading, set_uploading) = signal(false);
    let (upload_error, set_upload_error) = signal(String::new());

    let handle_upload = move |_| {
        let title_val = title.get().trim().to_string();
        let content_val = content.get().trim().to_string();
        
        if title_val.is_empty() || content_val.is_empty() {
            set_upload_error.set("Title and content are required".to_string());
            return;
        }

        set_uploading.set(true);
        set_upload_error.set(String::new());

        spawn_local(async move {
            let tag_list: Vec<String> = tags.get()
                .split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect();

            let request = DocumentUploadRequest {
                title: title_val,
                content: content_val,
                document_type: doc_type.get(),
                metadata: Some(json!({"source": "manual_upload"})),
                tags: if tag_list.is_empty() { None } else { Some(tag_list) },
            };

            match RagApiClient::upload_document(request).await {
                Ok(_) => {
                    on_upload_success.call(());
                }
                Err(e) => {
                    set_upload_error.set(format!("Upload failed: {}", e));
                    set_uploading.set(false);
                }
            }
        });
    };

    view! {
        <div class="bg-white/5 border border-white/10 rounded-lg p-6">
            <div class="flex justify-between items-center mb-4">
                <h5 class="text-lg font-medium">"Upload New Document"</h5>
                <button
                    on:click=move |_| on_cancel.call(())
                    class="text-white/60 hover:text-white/80"
                >
                    "✕"
                </button>
            </div>

            {move || {
                let error_msg = upload_error.get();
                if !error_msg.is_empty() {
                    view! {
                        <div class="bg-red-600/20 border border-red-500/30 rounded p-3 mb-4">
                            <p class="text-red-300 text-sm">{error_msg}</p>
                        </div>
                    }.into()
                } else {
                    view! { <div></div> }.into()
                }
            }}

            <div class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-white/80 mb-2">"Title"</label>
                    <input
                        type="text"
                        placeholder="Document title..."
                        class="w-full px-3 py-2 bg-white/10 border border-white/20 rounded text-white placeholder-white/50 focus:outline-none focus:border-purple-500"
                        prop:value=move || title.get()
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-white/80 mb-2">"Type"</label>
                    <select
                        class="w-full px-3 py-2 bg-white/10 border border-white/20 rounded text-white focus:outline-none focus:border-purple-500"
                        on:change=move |ev| {
                            match event_target_value(&ev).as_str() {
                                "fitness_guide" => set_doc_type.set(DocumentType::FitnessGuide),
                                "nutrition_info" => set_doc_type.set(DocumentType::NutritionInfo),
                                "exercise_description" => set_doc_type.set(DocumentType::ExerciseDescription),
                                "research_paper" => set_doc_type.set(DocumentType::ResearchPaper),
                                "user_manual" => set_doc_type.set(DocumentType::UserManual),
                                "faq" => set_doc_type.set(DocumentType::FAQ),
                                _ => set_doc_type.set(DocumentType::FitnessGuide),
                            }
                        }
                    >
                        <option value="fitness_guide">"🏋️ Fitness Guide"</option>
                        <option value="nutrition_info">"🍎 Nutrition Info"</option>
                        <option value="exercise_description">"💪 Exercise Description"</option>
                        <option value="research_paper">"📄 Research Paper"</option>
                        <option value="user_manual">"📖 User Manual"</option>
                        <option value="faq">"❓ FAQ"</option>
                    </select>
                </div>

                <div>
                    <label class="block text-sm font-medium text-white/80 mb-2">"Tags (comma separated)"</label>
                    <input
                        type="text"
                        placeholder="strength, beginner, upper body..."
                        class="w-full px-3 py-2 bg-white/10 border border-white/20 rounded text-white placeholder-white/50 focus:outline-none focus:border-purple-500"
                        prop:value=move || tags.get()
                        on:input=move |ev| set_tags.set(event_target_value(&ev))
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-white/80 mb-2">"Content"</label>
                    <textarea
                        placeholder="Enter document content..."
                        rows="8"
                        class="w-full px-3 py-2 bg-white/10 border border-white/20 rounded text-white placeholder-white/50 focus:outline-none focus:border-purple-500"
                        prop:value=move || content.get()
                        on:input=move |ev| set_content.set(event_target_value(&ev))
                    ></textarea>
                </div>

                <div class="flex gap-3">
                    <button
                        on:click=handle_upload
                        disabled=uploading
                        class="flex-1 px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-purple-600/50 text-white rounded transition-colors"
                    >
                        {move || if uploading.get() { "Uploading..." } else { "Upload Document" }}
                    </button>
                    <button
                        on:click=move |_| on_cancel.call(())
                        class="px-4 py-2 bg-white/10 hover:bg-white/20 border border-white/20 rounded transition-colors"
                    >
                        "Cancel"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DocumentList(
    documents: Vec<Document>,
    selected_documents: Signal<Vec<String>>,
    on_document_select: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            {documents.into_iter().map(|doc| {
                let doc_id = doc.id.clone();
                let is_selected = move || selected_documents.get().contains(&doc_id);
                
                view! {
                    <DocumentCard
                        document=doc
                        is_selected=is_selected.into()
                        on_select=Callback::new(move |_| on_document_select.call(doc_id.clone()))
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn DocumentCard(
    document: Document,
    is_selected: Signal<bool>,
    on_select: Callback<()>,
) -> impl IntoView {
    let doc_type_info = match document.document_type {
        DocumentType::FitnessGuide => ("🏋️", "Fitness Guide"),
        DocumentType::NutritionInfo => ("🍎", "Nutrition"),
        DocumentType::ExerciseDescription => ("💪", "Exercise"),
        DocumentType::ResearchPaper => ("📄", "Research"),
        DocumentType::UserManual => ("📖", "Manual"),
        DocumentType::FAQ => ("❓", "FAQ"),
    };

    let status_info = match document.embedding_status {
        EmbeddingStatus::Complete => ("✅", "Ready", "text-green-400"),
        EmbeddingStatus::Processing => ("⏳", "Processing", "text-yellow-400"),
        EmbeddingStatus::Pending => ("⏸️", "Pending", "text-gray-400"),
        EmbeddingStatus::Failed => ("❌", "Failed", "text-red-400"),
    };

    view! {
        <div class=move || format!(
            "flex items-center gap-4 p-4 border rounded-lg transition-all cursor-pointer hover:border-white/30 {}",
            if is_selected.get() {
                "bg-purple-500/20 border-purple-500/50"
            } else {
                "bg-white/5 border-white/10"
            }
        )>
            <input
                type="checkbox"
                class="w-4 h-4"
                prop:checked=move || is_selected.get()
                on:change=move |_| on_select.call(())
            />
            
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-3 mb-2">
                    <span class="text-lg">{doc_type_info.0}</span>
                    <h5 class="font-medium text-white truncate">{document.title}</h5>
                    <span class="text-xs bg-white/20 px-2 py-1 rounded">{doc_type_info.1}</span>
                </div>
                
                <p class="text-white/70 text-sm line-clamp-2 mb-2">
                    {if document.content.len() > 150 {
                        format!("{}...", &document.content[..150])
                    } else {
                        document.content
                    }}
                </p>
                
                <div class="flex items-center justify-between text-xs text-white/50">
                    <div class="flex items-center gap-4">
                        <span>
                            "Created: " {document.created_at.split('T').next().unwrap_or(&document.created_at)}
                        </span>
                        {if !document.tags.is_empty() {
                            view! {
                                <span>"Tags: " {document.tags.join(", ")}</span>
                            }.into()
                        } else {
                            view! { <span></span> }.into()
                        }}
                    </div>
                    <div class=format!("flex items-center gap-1 {}", status_info.2)>
                        <span>{status_info.0}</span>
                        <span>{status_info.1}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn BulkOperations(
    selected_count: usize,
    on_delete: Callback<()>,
    on_clear: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="flex gap-2">
            <button
                on:click=move |_| on_delete.call(())
                class="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded transition-colors"
            >
                "🗑️ Delete"
            </button>
            <button
                on:click=move |_| on_clear.call(())
                class="px-3 py-1 bg-white/10 hover:bg-white/20 border border-white/20 text-white text-sm rounded transition-colors"
            >
                "Clear Selection"
            </button>
        </div>
    }
}

// Helper function to load documents
fn load_documents(
    doc_type: Option<DocumentType>,
    set_documents: WriteSignal<Vec<Document>>,
    set_loading: WriteSignal<bool>,
    set_error: WriteSignal<String>,
) {
    set_loading.set(true);
    set_error.set(String::new());

    spawn_local(async move {
        match RagApiClient::get_documents(doc_type).await {
            Ok(docs) => {
                set_documents.set(docs);
            }
            Err(e) => {
                // Fallback to sample documents on error
                let sample_docs = create_sample_documents();
                set_documents.set(sample_docs);
                set_error.set(format!("Using sample data: {}", e));
            }
        }
        set_loading.set(false);
    });
}

// Sample documents for demo purposes
fn create_sample_documents() -> Vec<Document> {
    vec![
        Document {
            id: "1".to_string(),
            title: "Complete Guide to Compound Exercises".to_string(),
            content: "Compound exercises are multi-joint movements that work multiple muscle groups simultaneously. Examples include squats, deadlifts, bench press, and pull-ups. These exercises are highly effective for building strength and muscle mass...".to_string(),
            document_type: DocumentType::FitnessGuide,
            metadata: json!({"difficulty": "intermediate", "duration": "45min"}),
            tags: vec!["strength".to_string(), "compound".to_string(), "muscle-building".to_string()],
            created_at: "2024-01-15T10:30:00Z".to_string(),
            updated_at: "2024-01-15T10:30:00Z".to_string(),
            embedding_status: EmbeddingStatus::Complete,
        },
        Document {
            id: "2".to_string(),
            title: "Macronutrient Basics for Athletes".to_string(),
            content: "Understanding macronutrients is crucial for athletic performance. Proteins provide amino acids for muscle repair, carbohydrates fuel workouts, and fats support hormone production...".to_string(),
            document_type: DocumentType::NutritionInfo,
            metadata: json!({"target_audience": "athletes", "level": "beginner"}),
            tags: vec!["nutrition".to_string(), "macros".to_string(), "performance".to_string()],
            created_at: "2024-01-14T14:20:00Z".to_string(),
            updated_at: "2024-01-14T14:20:00Z".to_string(),
            embedding_status: EmbeddingStatus::Complete,
        },
    ]
}
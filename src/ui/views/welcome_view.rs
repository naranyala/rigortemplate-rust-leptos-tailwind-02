use leptos::prelude::*;

#[component]
pub fn WelcomeView() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center">
            <div class="text-center space-y-4">
                <h1 class="text-4xl font-bold text-surface-800 dark:text-surface-100">
                    "Welcome to the App"
                </h1>
                <p class="text-surface-500 dark:text-surface-400">
                    "Select a page from the sidebar to get started."
                </p>
            </div>
        </div>
    }
}

use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn SettingsView() -> impl IntoView {
    view! {
        <div class="p-6 max-w-2xl space-y-8">
            <h2 class="text-2xl font-bold text-slate-800">"Account Settings"</h2>
            
            <div class="space-y-6 bg-white p-6 rounded-2xl border border-slate-200 shadow-sm">
                <h3 class="text-sm font-semibold text-slate-400 uppercase tracking-wider">"Profile Information"</h3>
                <div class="grid grid-cols-1 gap-4">
                    <div class="space-y-1">
                        <label class="text-sm font-medium text-slate-700">"Display Name"</label>
                        <input type="text" class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:ring-2 focus:ring-indigo-500 outline-none transition-all" value="Leptos User" />
                    </div>
                    <div class="space-y-1">
                        <label class="text-sm font-medium text-slate-700">"Email Address"</label>
                        <input type="email" class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:ring-2 focus:ring-indigo-500 outline-none transition-all" value="user@example.com" />
                    </div>
                </div>
            </div>

            <div class="space-y-6 bg-white p-6 rounded-2xl border border-slate-200 shadow-sm">
                <h3 class="text-sm font-semibold text-slate-400 uppercase tracking-wider">"Preferences"</h3>
                <div class="flex items-center justify-between py-2">
                    <div>
                        <p class="text-sm font-medium text-slate-700">"Email Notifications"</p>
                        <p class="text-xs text-slate-500">"Receive daily summary emails"</p>
                    </div>
                    <input type="checkbox" class="w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500" checked />
                </div>
                <div class="flex items-center justify-between py-2">
                    <div>
                        <p class="text-sm font-medium text-slate-700">"Dark Mode"</p>
                        <p class="text-xs text-slate-500">"Switch to dark theme across app"</p>
                    </div>
                    <input type="checkbox" class="w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500" />
                </div>
            </div>

            <div class="flex justify-end">
                <button class="bg-indigo-600 text-white px-6 py-2 rounded-lg font-medium hover:bg-indigo-700 transition-colors">
                    "Save Changes"
                </button>
            </div>
        </div>
    }
}

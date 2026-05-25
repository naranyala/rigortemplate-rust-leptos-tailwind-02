use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::ui::layout::MainLayout;
use crate::ui::views::{AccordionView, BadgeView, ButtonView, CardView, DataTableView, ModalView, MultiSelectView, PanelView, ProgressView, StatsCardView, WelcomeView, hooks::{ToggleView, MountedView, ClipboardView, DebounceView, IntervalView, LocalStorageView, WindowSizeView}};
use crate::shared::theme::provide_theme_context;
use crate::ui::shared::toasts::provide_toast_context;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme_context();
    provide_toast_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <MainLayout>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=WelcomeView />
                    <Route path=path!("/toggle") view=ToggleView />
                    <Route path=path!("/mounted") view=MountedView />
                    <Route path=path!("/clipboard") view=ClipboardView />
                    <Route path=path!("/debounce") view=DebounceView />
                    <Route path=path!("/interval") view=IntervalView />
                    <Route path=path!("/local-storage") view=LocalStorageView />
                    <Route path=path!("/window-size") view=WindowSizeView />
                    <Route path=path!("/button") view=ButtonView />
                    <Route path=path!("/badge") view=BadgeView />
                    <Route path=path!("/card") view=CardView />
                    <Route path=path!("/modal") view=ModalView />
                    <Route path=path!("/progress") view=ProgressView />
                    <Route path=path!("/stats") view=StatsCardView />
                    <Route path=path!("/data-table") view=DataTableView />
                    <Route path=path!("/multi-select") view=MultiSelectView />
                    <Route path=path!("/accordion") view=AccordionView />
                    <Route path=path!("/panel") view=PanelView />
                </Routes>
            </MainLayout>
        </Router>
    }
}

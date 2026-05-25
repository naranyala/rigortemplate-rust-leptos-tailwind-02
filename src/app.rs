use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::ui::layout::MainLayout;
use crate::ui::views::{AccordionView, PanelView, WelcomeView, hooks::{ToggleView, MountedView, ClipboardView}};
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
                    <Route path=path!("/accordion") view=AccordionView />
                    <Route path=path!("/panel") view=PanelView />
                </Routes>
            </MainLayout>
        </Router>
    }
}

use leptos::prelude::*;
use leptos_meta::*;

use crate::components::*;
use crate::core::state::provide_global_state;
use crate::shared::theme::provide_theme_context;
use crate::core::provider::ServiceProvider;



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppRoute {
    Hooks,
    Accordion,
    Panel,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_global_state();
    provide_theme_context();
    ServiceProvider::provide_all();
    
    let (route, set_route) = signal(AppRoute::Accordion);
    provide_context(route);
    provide_context(set_route);

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <MainLayout>
            {move || match route.get() {
                AppRoute::Hooks => view! { <HooksCatalogView /> }.into_any(),
                AppRoute::Accordion => view! { <AccordionView /> }.into_any(),
                AppRoute::Panel => view! { <PanelView /> }.into_any(),
            }}
        </MainLayout>
    }
}

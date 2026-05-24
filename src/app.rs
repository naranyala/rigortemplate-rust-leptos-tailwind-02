use leptos::prelude::*;
use leptos_meta::*;

use crate::components::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppRoute {
    Catalog,
    Hooks,
    Accordion,
    Panel,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    let (route, set_route) = signal(AppRoute::Accordion);
    provide_context(set_route);

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <MainLayout route=route.into() />
    }
}

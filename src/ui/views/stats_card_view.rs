use leptos::prelude::*;
use crate::ui::shared::stats_card::StatsCard;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn StatsCardView() -> impl IntoView {
    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Stats Card Component" 
                description="Compact metric displays for dashboards and analytics overviews."
                code=r#"view! {
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatsCard title="Total Users" value="12,450" trend="+12%" color="bg-green-100 text-green-700" />
        <StatsCard title="Revenue" value="$45,200" trend="+8%" color="bg-green-100 text-green-700" />
        <StatsCard title="Churn Rate" value="2.4%" trend="-0.5%" color="bg-red-100 text-red-700" />
        <StatsCard title="Active Sessions" value="1,120" trend="+15%" color="bg-blue-100 text-blue-700" />
    </div>
}"#
            >
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <StatsCard title="Total Users" value="12,450" trend="+12%" color="bg-green-100 text-green-700" />
                    <StatsCard title="Revenue" value="$45,200" trend="+8%" color="bg-green-100 text-green-700" />
                    <StatsCard title="Churn Rate" value="2.4%" trend="-0.5%" color="bg-red-100 text-red-700" />
                    <StatsCard title="Active Sessions" value="1,120" trend="+15%" color="bg-blue-100 text-blue-700" />
                </div>
            </DemoPage>
        </div>
    }
}

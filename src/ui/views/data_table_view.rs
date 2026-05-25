use leptos::prelude::*;
use crate::ui::shared::data_table::{DataTable, Column};
use crate::ui::shared::demo_page::DemoPage;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    role: String,
}

#[component]
pub fn DataTableView() -> impl IntoView {
    let users = signal(vec![
        User { name: "Alice Smith".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string() },
        User { name: "Bob Jones".to_string(), email: "bob@example.com".to_string(), role: "User".to_string() },
        User { name: "Charlie Brown".to_string(), email: "charlie@example.com".to_string(), role: "Editor".to_string() },
        User { name: "Diana Prince".to_string(), email: "diana@example.com".to_string(), role: "User".to_string() },
    ]);

    let columns = vec![
        Column { header: "Name".to_string(), accessor: |u: &User| u.name.clone(), sort_key: Some("name") },
        Column { header: "Email".to_string(), accessor: |u: &User| u.email.clone(), sort_key: Some("email") },
        Column { header: "Role".to_string(), accessor: |u: &User| u.role.clone(), sort_key: Some("role") },
    ];

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Data Table Component" 
                description="A powerful, sortable data table for managing large sets of structured information."
                code=r#"view! {
    let users = signal(vec![
        User { name: "Alice".into(), email: "alice@example.com".into(), role: "Admin".into() },
        User { name: "Bob".into(), email: "bob@example.com".into(), role: "User".into() },
    ]);
    let columns = vec![
        Column { header: "Name".into(), accessor: |u| u.name.clone(), sort_key: Some("name") },
        Column { header: "Email".into(), accessor: |u| u.email.clone(), sort_key: Some("email") },
        Column { header: "Role".into(), accessor: |u| u.role.clone(), sort_key: Some("role") },
    ];
    <DataTable data=users columns=columns />
}"#
            >
                <DataTable data=users.0 columns=columns />
            </DemoPage>
        </div>
    }
}

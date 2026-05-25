use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct Column<T> {
    pub header: String,
    pub accessor: fn(&T) -> String,
    pub sort_key: Option<&'static str>,
}

#[component]
pub fn DataTable<T>(
    data: ReadSignal<Vec<T>>,
    columns: Vec<Column<T>>,
) -> impl IntoView 
where 
    T: Clone + Send + Sync + PartialEq + 'static
{
    let (sort_col, set_sort_col) = signal(None::<usize>);
    let (sort_asc, set_sort_asc) = signal(true);

    let columns_for_memo = columns.clone();
    let sorted_data = Memo::new(move |_| {
        let mut d = data.get();
        if let Some(col_idx) = sort_col.get() {
            let col = &columns_for_memo[col_idx];
            d.sort_by(|a, b| {
                let a_val = (col.accessor)(a);
                let b_val = (col.accessor)(b);
                if sort_asc.get() {
                    a_val.cmp(&b_val)
                } else {
                    b_val.cmp(&a_val)
                }
            });
        }
        d
    });

    view! {
        <div class="overflow-x-auto rounded-lg border border-gray-200 dark:border-gray-700">
            <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700 bg-white dark:bg-gray-900 text-sm text-left">
                <thead class="bg-gray-50 dark:bg-gray-800 text-gray-700 dark:text-gray-300 font-medium">
                    <tr>
                {columns.iter().enumerate().map(|(i, col)| {
                    view! {
                        <th 
                            class="px-4 py-3 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                            on:click=move |_| {
                                if sort_col.get() == Some(i) {
                                    set_sort_asc.update(|v| *v = !*v);
                                } else {
                                    set_sort_col.set(Some(i));
                                    set_sort_asc.set(true);
                                }
                            }
                        >
                            <div class="flex items-center gap-2">
                                {col.header.clone()}
                                {move || {
                                    if sort_col.get() == Some(i) {
                                        if sort_asc.get() { "↑" } else { "↓" }
                                    } else {
                                        "↕"
                                    }
                                }}
                            </div>
                        </th>
                    }
                }).collect::<Vec<_>>()}

                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                    {move || {
                        sorted_data.get().into_iter().map(|row| {
                            view! {
                                <tr class="hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">
                                    {columns.iter().map(|col| {
                                        view! {
                                            <td class="px-4 py-3 text-gray-600 dark:text-gray-400">
                                                {(col.accessor)(&row)}
                                            </td>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </table>
        </div>
    }
}

use leptos::prelude::*;
use crate::ui::shared::accordion::{Accordion, AccordionItem};
use crate::ui::shared::code_block::CodeBlock;

#[component]
pub fn AccordionView() -> impl IntoView {
    view! {
        <div class="min-h-screen p-8 lg:p-16 max-w-3xl mx-auto space-y-12">
            <div class="relative">
                <div class="absolute -inset-1 bg-gradient-to-r from-accent-500 to-purple-600 rounded-[2rem] blur opacity-20"></div>
                <div class="relative space-y-6">
                    <Accordion>
                        <AccordionItem title="The Architecture".to_string()>
                            <p class="leading-relaxed">
                                "Built with a modular approach, our Accordion uses a state-driven model where each item 
                                manages its own open/closed state, ensuring optimal performance and zero layout shift."
                            </p>
                        </AccordionItem>
                        <AccordionItem title="Performance Optimized".to_string()>
                            <p class="leading-relaxed">
                                "Leveraging Leptos' fine-grained reactivity, only the affected DOM elements are updated 
                                when an item is toggled, resulting in near-instant response times."
                            </p>
                        </AccordionItem>
                        <AccordionItem title="Tailwind-Powered".to_string()>
                            <p class="leading-relaxed">
                                "Every detail, from the subtle borders to the smooth transitions, is crafted using 
                                Tailwind CSS utility classes for a modern, consistent aesthetic."
                            </p>
                        </AccordionItem>
                    </Accordion>
                    <div class="bg-surface-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                        <div class="flex items-center justify-between mb-4">
                            <span class="text-xs font-mono text-surface-400 uppercase tracking-widest">"Source Code"</span>
                        </div>
                        <CodeBlock code=r#"view! {
    <Accordion>
        <AccordionItem title="The Architecture".to_string()>
            <p class="leading-relaxed">
                "Built with a modular approach, our Accordion uses a state-driven model where each item 
                manages its own open/closed state, ensuring optimal performance and zero layout shift."
            </p>
        </AccordionItem>
        <AccordionItem title="Performance Optimized".to_string()>
            <p class="leading-relaxed">
                "Leveraging Leptos' fine-grained reactivity, only the affected DOM elements are updated 
                when an item is toggled, resulting in near-instant response times."
            </p>
        </AccordionItem>
        <AccordionItem title="Tailwind-Powered".to_string()>
            <p class="leading-relaxed">
                "Every detail, from the subtle borders to the smooth transitions, is crafted using 
                Tailwind CSS utility classes for a modern, consistent aesthetic."
            </p>
        </AccordionItem>
    </Accordion>
}"# />
                    </div>
                </div>
            </div>
        </div>
    }
}

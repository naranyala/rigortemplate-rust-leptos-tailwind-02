use leptos::prelude::*;
use crate::shared::accordion::{Accordion, AccordionItem};
use crate::shared::code_block::CodeBlock;

#[component]
pub fn AccordionView() -> impl IntoView {
    view! {
        <div class="min-h-screen p-8 lg:p-16 max-w-3xl mx-auto space-y-12">
            <div class="space-y-4 text-center">
                <div class="inline-flex items-center px-3 py-1 rounded-full bg-indigo-50 text-indigo-600 text-xs font-bold uppercase tracking-wider mb-4">
                    "UI Primitive"
                </div>
                <h2 class="text-4xl font-extrabold text-slate-900 tracking-tight">"Interactive Accordions"</h2>
                <p class="text-lg text-slate-500 max-w-xl mx-auto leading-relaxed">
                    "A sophisticated way to organize content and reduce cognitive load by revealing information on demand."
                </p>
            </div>
            
                <div class="relative">
                    <div class="absolute -inset-1 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-[2rem] blur opacity-20"></div>
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
                        <div class="bg-slate-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                            <div class="flex items-center justify-between mb-4">
                                <span class="text-xs font-mono text-slate-400 uppercase tracking-widest">"Source Code"</span>
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

            <div class="bg-slate-50 rounded-3xl p-8 border border-slate-100 text-center space-y-4">
                <h4 class="font-bold text-slate-800">"Ready to implement?"</h4>
                <p class="text-slate-600 text-sm">"Copy the primitives from our stdlib to your project in seconds."</p>
                <button class="px-5 py-2 bg-white border border-slate-200 rounded-xl text-sm font-semibold text-slate-700 hover:bg-slate-50 transition-all">
                    "View Documentation"
                </button>
            </div>
        </div>
    }
}

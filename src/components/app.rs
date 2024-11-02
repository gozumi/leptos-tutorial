use super::demo_content::DemoContent;
use leptos::{component, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
  view! {
    <section>
      <DemoContent />
    </section>
  }
}

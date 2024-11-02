use leptos::{component, create_signal, view, IntoView};

#[component]
pub fn DemoContent() -> impl IntoView {
  let (count, set_count) = create_signal(0);

  view! {
    <button on:click=move |_| {
      set_count(count() + 4);
    }>
      // on stable, this is move || count.get();
      "Click me: " {move || count()}
    </button>
  }
}

use yew::prelude::*;
use yew_hooks::use_title;

use crate::components::{AlertDisplay, Header, QuestionsList, SubmitButton};
use crate::state::{AppContext, AppState};

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer_eq(AppState::default);
    let context = AppContext::new(state.clone());

    {
        let context = context.clone();
        use_effect_with((), move |()| {
            context.get_quiz();
        });
    }

    use_title(state.header.clone());

    html! {
        <ContextProvider<AppContext> context={context}>
            <div class="min-h-screen bg-gray-50">
                <main class="container mx-auto px-4 py-12 max-w-4xl">
                    <Header />
                    <AlertDisplay />
                    <QuestionsList />
                    <div class="mt-6">
                        <SubmitButton />
                    </div>
                </main>
            </div>
        </ContextProvider<AppContext>>
    }
}

use yew::prelude::*;
use yew_bootstrap::{
    component::{Column, Container, ContainerSize, Row},
    util::include_cdn,
};
use yew_hooks::use_title;

use crate::components::{AlertDisplay, Header, QuestionsList};
use crate::state::{AppContext, State};

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer_eq(State::default);
    let context = AppContext::new(state.clone());

    {
        let context = context.clone();
        use_effect_with((), move |()| {
            context.fetch_info();
        });
    }

    use_title(state.header.clone());

    let onsubmit = {
        let context = context.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            context.submit_answers();
        })
    };

    html! {
        <ContextProvider<AppContext> context={context}>
            { include_cdn() }
            <Container size={ContainerSize::ExtraSmall}>
                <Row class="vh-100 align-items-center">
                    <Column md={8} class="mx-auto">
                        <div class="text-center my-4">
                            <Header />
                            <AlertDisplay />
                            <from {onsubmit}>
                                <QuestionsList />
                            </from>
                        </div>
                    </Column>
                </Row>
            </Container>
        </ContextProvider<AppContext>>
    }
}

use yew::prelude::*;
use yew_bootstrap::component::{Column, Container, ContainerSize, Row};
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
            <link
                href="https://lf26-cdn-tos.bytecdntp.com/cdn/expire-1-M/bootstrap/5.1.3/css/bootstrap.css"
                type="text/css"
                rel="stylesheet"
            />
            <Container size={ContainerSize::ExtraSmall}>
                <Row class="vh-100 align-items-center">
                    <Column md={8} class="mx-auto">
                        <div class="text-center my-4">
                            <Header />
                            <AlertDisplay />
                            <QuestionsList />
                            <SubmitButton />
                        </div>
                    </Column>
                </Row>
            </Container>
        </ContextProvider<AppContext>>
    }
}

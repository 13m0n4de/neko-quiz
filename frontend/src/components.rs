use crate::state::{use_app_context, AppAction};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_bootstrap::component::{
    card::{Card, CardText},
    form::{FormControl, FormControlType},
    Alert, Button, ButtonSize,
};
use yew_bootstrap::util::Color;

#[function_component(Header)]
pub fn header() -> Html {
    let context = use_app_context();
    html! {
        <h1 class="mb-4">{ &context.state.header }</h1>
    }
}

#[function_component(AlertDisplay)]
pub fn alert_display() -> Html {
    let context = use_app_context();
    if let Some(info) = context.state.alert_info.clone() {
        let text_vnode = Html::from_html_unchecked(AttrValue::from(info.text));
        html! { <Alert style={info.color} children={text_vnode} /> }
    } else {
        html! {}
    }
}

#[function_component(QuestionsList)]
pub fn questions_list() -> Html {
    let context = use_app_context();

    let onchange = {
        let state = context.state.clone();
        Callback::from(move |event: Event| {
            let target: HtmlInputElement = event.target_unchecked_into();
            state.dispatch(AppAction::SetAnswer(target.name(), target.value()));
        })
    };

    html! {
        <>
            { for context.state.questions.iter().enumerate().map(|(idx, question)| {
                let stored_answer = context.state.answers.borrow().get(&question.id).cloned().unwrap_or_default();

                let text = Html::from_html_unchecked(AttrValue::from(question.text.clone()));
                let hint = Html::from_html_unchecked(AttrValue::from(question.hint.clone()));

                html! {
                    <Card body=true class="mb-4 text-start">
                        <CardText class="mb-0">
                            <span>{ format!("{}. ", idx + 1) }</span>
                            { text }
                            <b>{ format!("（{} 分）", &question.points) }</b>
                        </CardText>
                        <small class="text-muted">{ "提示：" }{ hint }</small>
                        <FormControl
                            id={ format!("question-{}", idx + 1) }
                            name={ question.id.clone() }
                            ctype={ FormControlType::Text }
                            class="my-2"
                            value={ stored_answer }
                            onchange={ onchange.clone() }
                        />
                    </Card>
                }
            }) }
        </>
    }
}

#[function_component(SubmitButton)]
pub fn submit_button() -> Html {
    let context = use_app_context();

    let onclick = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            context.create_submission();
        })
    };

    html! {
        <Button
            class="w-100"
            style={Color::Primary}
            block={true}
            size={ButtonSize::Large}
            text="提交"
            {onclick}
        />
    }
}

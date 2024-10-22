use crate::{
    models::AlertType,
    state::{use_app_context, AppAction},
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let context = use_app_context();
    html! {
        <h1 class="text-3xl font-semibold text-gray-900 mb-5 text-center">
            { &context.state.header }
        </h1>
    }
}

#[function_component(AlertDisplay)]
pub fn alert_display() -> Html {
    let context = use_app_context();
    if let Some(info) = context.state.alert_info.clone() {
        let text_vnode = Html::from_html_unchecked(AttrValue::from(info.text));

        let (bg_color, border_color, text_color) = match info.alert_type {
            AlertType::Success => ("bg-teal-600/10", "border-teal-600/25", "text-teal-800"),
            AlertType::Error => ("bg-red-50", "border-red-200", "text-red-800"),
            AlertType::Info => ("bg-gray-100", "border-gray-300", "text-gray-800"),
        };

        html! {
            <div class={classes!(
                "p-4",
                "mb-6",
                "rounded-md",
                "border",
                bg_color,
                border_color,
                text_color,
                "text-center"
            )}>
                { text_vnode }
            </div>
        }
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
        <div class="space-y-4">
            { for context.state.questions.iter().enumerate().map(|(idx, question)| {
                let stored_answer = context.state.answers.borrow().get(&question.id).cloned().unwrap_or_default();
                let text = Html::from_html_unchecked(AttrValue::from(question.text.clone()));
                let hint = Html::from_html_unchecked(AttrValue::from(question.hint.clone()));

                html! {
                    <div class="border border-gray-300/70 rounded-md p-4 bg-white">
                        <div>
                            <div class="text-gray-900">
                                <span class="text-teal-700 mr-2">{ format!("{}.", idx + 1) }</span>
                                { text }
                                <span class="text-sm text-teal-700 ml-2">
                                    { format!("（{} 分）", &question.points) }
                                </span>
                            </div>
                            <div class="text-sm text-gray-500 mt-1 mb-3">
                                { hint }
                            </div>
                        </div>
                        <input
                            type="text"
                            id={ format!("question-{}", idx + 1) }
                            name={ question.id.clone() }
                            class="w-full px-3 py-2 rounded-md border border-gray-200/70
                                   focus:outline-none focus:border-teal-700
                                   bg-white text-gray-900 transition-colors"
                            value={ stored_answer }
                            onchange={ onchange.clone() }
                        />
                    </div>
                }
            }) }
        </div>
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
        <button
            class="w-full bg-teal-600 hover:bg-teal-700 text-white font-medium
                   py-2 px-4 rounded transition-colors focus:outline-none"
            onclick={onclick}
        >
            { "提交" }
        </button>
    }
}

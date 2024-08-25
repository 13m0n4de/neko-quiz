use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew_bootstrap::{
    component::{
        card::{Card, CardText},
        form::{FormControl, FormControlType},
        Alert, Button, ButtonSize, Column, Container, ContainerSize, Row,
    },
    util::{include_cdn, Color},
};
use yew_hooks::use_title;

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[derive(Deserialize, PartialEq, Clone)]
struct Info {
    title: String,
    questions: Vec<Question>,
}

#[derive(Deserialize, PartialEq, Clone)]
struct Question {
    id: String,
    text: String,
    points: u32,
    hint: String,
}

#[derive(PartialEq, Clone)]
struct AlertInfo {
    color: Color,
    text: String,
}

#[derive(Serialize)]
struct Answer {
    id: String,
    answer: String,
}

#[derive(Deserialize, PartialEq, Clone)]
struct QuizResponse {
    status: bool,
    score: u8,
    message: String,
}

#[derive(PartialEq, Clone)]
struct State {
    header: String,
    questions: Vec<Question>,
    answers: HashMap<String, String>,
    alert_info: Option<AlertInfo>,
}

enum Action {
    Info(Info),
    Answer(String, String),
    AlertInfo(Option<AlertInfo>),
    QuizResponse(QuizResponse),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Info(info) => Rc::new(State {
                header: info.title,
                questions: info.questions,
                ..(*self).clone()
            }),
            Action::Answer(id, answer) => {
                let mut new_state = (*self).clone();
                new_state.answers.insert(id.clone(), answer.clone());
                LocalStorage::set(id, &answer).unwrap_or_default();
                Rc::new(new_state)
            }
            Action::AlertInfo(info) => Rc::new(State {
                alert_info: info,
                ..(*self).clone()
            }),
            Action::QuizResponse(response) => {
                let color = if response.status {
                    Color::Success
                } else {
                    Color::Secondary
                };
                Rc::new(State {
                    alert_info: Some(AlertInfo {
                        color,
                        text: format!(
                            "本次测验总得分为 {}。<br/>{}",
                            response.score, response.message
                        ),
                    }),
                    ..(*self).clone()
                })
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct AppContext {
    state: UseReducerHandle<State>,
}

impl AppContext {
    fn new(state: UseReducerHandle<State>) -> Self {
        Self { state }
    }
}

#[hook]
fn use_app_context() -> AppContext {
    use_context::<AppContext>().expect("No context found!")
}

async fn get_info() -> Result<Info, String> {
    let response = Request::get("/api/info")
        .send()
        .await
        .map_err(|_| "Failed to get page information.")?;

    response
        .json::<Info>()
        .await
        .map_err(|_| "Failed to parse server response.".to_string())
}

async fn submit_answers(answers_map: &HashMap<String, String>) -> Result<QuizResponse, String> {
    let answers_data: Vec<Answer> = answers_map
        .iter()
        .map(|(id, answer)| Answer {
            id: id.clone(),
            answer: answer.clone(),
        })
        .collect();

    let request = Request::post("/api/answers")
        .json(&answers_data)
        .map_err(|_| "Failed to serialize JSON.")?;

    let response = request.send().await.map_err(|_| "Failed to send answer.")?;

    response
        .json::<QuizResponse>()
        .await
        .map_err(|_| "Failed to parse server response.".to_string())
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer_eq(|| State {
        header: String::new(),
        questions: Vec::new(),
        answers: HashMap::new(),
        alert_info: None,
    });

    {
        let state = state.clone();
        use_effect_with((), move |()| {
            let stored_answers: HashMap<String, String> =
                LocalStorage::get_all().unwrap_or_default();
            for (key, value) in stored_answers {
                state.dispatch(Action::Answer(key, value));
            }

            spawn_local(async move {
                match get_info().await {
                    Ok(info) => state.dispatch(Action::Info(info)),
                    Err(err) => state.dispatch(Action::AlertInfo(Some(AlertInfo {
                        color: Color::Danger,
                        text: err,
                    }))),
                }
            });
        });
    }

    use_title(state.header.clone());

    let context = AppContext::new(state);

    html! {
        <ContextProvider<AppContext> context={context}>
            { include_cdn() }
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

#[function_component(Header)]
fn header() -> Html {
    let context = use_app_context();
    html! {
        <h1 class="mb-4">{ context.state.header.clone() }</h1>
    }
}

#[function_component(AlertDisplay)]
fn alert_display() -> Html {
    let context = use_app_context();
    if let Some(info) = context.state.alert_info.clone() {
        let text_vnode = Html::from_html_unchecked(AttrValue::from(info.text));
        html! { <Alert style={info.color} children={text_vnode} /> }
    } else {
        html! {}
    }
}

#[function_component(QuestionsList)]
fn questions_list() -> Html {
    let context = use_app_context();

    let oninput = {
        let state = context.state.clone();
        Callback::from(move |event: InputEvent| {
            let target: HtmlInputElement = event.target_unchecked_into();
            state.dispatch(Action::Answer(target.id(), target.value()));
        })
    };

    html! {
        <>
            { for context.state.questions.iter().enumerate().map(|(idx, question)| {
                let stored_answer = context.state.answers.get(&question.id).cloned().unwrap_or_default();

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
                            id={ question.id.clone() }
                            ctype={ FormControlType::Text }
                            class="my-2"
                            value={stored_answer}
                            oninput={ oninput.clone() }
                        />
                    </Card>
                }
            }) }
        </>
    }
}

#[function_component(SubmitButton)]
fn submit_button() -> Html {
    let context = use_app_context();

    let onclick = {
        let state = context.state.clone();
        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            let answers = state.answers.clone();
            spawn_local(async move {
                match submit_answers(&answers).await {
                    Ok(response) => state.dispatch(Action::QuizResponse(response)),
                    Err(err) => state.dispatch(Action::AlertInfo(Some(AlertInfo {
                        color: Color::Danger,
                        text: err,
                    }))),
                }
            });
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

fn main() {
    yew::Renderer::<App>::new().render();
}

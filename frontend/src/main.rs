use std::collections::HashMap;

use yew::prelude::*;

use yew_bootstrap::component::card::*;
use yew_bootstrap::component::form::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

use yew_hooks::{use_map, use_title, UseMapHandle};

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;

#[derive(Deserialize, PartialEq, Clone)]
struct Info {
    title: String,
    questions: Vec<Question>,
}

#[derive(Deserialize, PartialEq, Clone)]
struct Question {
    id: usize,
    text: String,
    points: u32,
    hint: String,
}

#[derive(Properties, Clone, PartialEq)]
struct QuestionsListProps {
    questions: UseStateHandle<Vec<Question>>,
    oninput: Callback<InputEvent>,
}

#[derive(Clone)]
struct AlertInfo {
    color: Color,
    text: String,
}

#[derive(Serialize)]
struct Answer {
    id: usize,
    answer: String,
}

#[derive(Deserialize, PartialEq, Clone)]
struct AnswersResponse {
    status: bool,
    score: u8,
    message: String,
}

#[function_component(QuestionsList)]
fn questions_list(props: &QuestionsListProps) -> Html {
    html! {
        <>
            { for props.questions.iter().map(|question| {
                let stored_answer: String = LocalStorage::get(question.id.to_string()).unwrap_or_default();

                let text = Html::from_html_unchecked(AttrValue::from(question.text.clone()));
                let hint = Html::from_html_unchecked(AttrValue::from(question.hint.clone()));

                html! {
                    <Card body=true class="mb-4 text-start">
                        <CardText class="mb-0">
                            <span>{ format!("{}. ", &question.id) }</span>
                            { text }
                            <b>{ format!("（{} 分）", &question.points) }</b>
                        </CardText>
                        <small class="text-muted">{ "提示：" }{ hint }</small>
                        <FormControl
                            id={ question.id.to_string() }
                            ctype={ FormControlType::Text }
                            class="my-2"
                            value={stored_answer}
                            oninput={ props.oninput.clone() }
                        />
                    </Card>
                }
            }) }
        </>
    }
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

async fn submit_answers(
    answers_map: UseMapHandle<usize, String>,
) -> Result<AnswersResponse, String> {
    let answers_data: Vec<Answer> = answers_map
        .current()
        .iter()
        .map(|(&id, answer)| Answer {
            id,
            answer: answer.clone(),
        })
        .collect();

    let request = Request::post("/api/answers")
        .json(&answers_data)
        .map_err(|_| "Failed to serialize JSON.")?;

    let response = request.send().await.map_err(|_| "Failed to send answer.")?;

    response
        .json::<AnswersResponse>()
        .await
        .map_err(|_| "Failed to parse server response.".to_string())
}

#[function_component(App)]
fn app() -> Html {
    let header = use_state(String::new);
    let questions = use_state(Vec::new);
    let answers = use_map(HashMap::<usize, String>::new());
    let alert_info = use_state(|| None::<AlertInfo>);

    {
        let header = header.clone();
        let questions = questions.clone();
        let answers = answers.clone();
        let alert_info = alert_info.clone();

        use_effect_with((), move |_| {
            answers.set(LocalStorage::get_all().unwrap_or_default());

            spawn_local(async move {
                match get_info().await {
                    Ok(info) => {
                        header.set(info.title);
                        questions.set(info.questions);
                    }
                    Err(err) => alert_info.set(Some(AlertInfo {
                        color: Color::Danger,
                        text: err,
                    })),
                }
            });
        });
    }

    use_title((*header).clone());

    let onclick = {
        let answers = answers.clone();
        let alert_info = alert_info.clone();

        Callback::from(move |_: MouseEvent| {
            let answers = answers.clone();
            let alert_info = alert_info.clone();

            spawn_local(async move {
                match submit_answers(answers).await {
                    Ok(result) => {
                        let color = if result.status {
                            Color::Success
                        } else {
                            Color::Secondary
                        };

                        alert_info.set(Some(AlertInfo {
                            color,
                            text: format!(
                                "本次测验总得分为 {}。<br/>{}",
                                result.score, result.message
                            ),
                        }));
                    }
                    Err(err) => alert_info.set(Some(AlertInfo {
                        color: Color::Danger,
                        text: err,
                    })),
                }
            });
        })
    };

    let oninput = {
        let answers = answers.clone();
        Callback::from(move |event: InputEvent| {
            let target: HtmlInputElement = event.target_unchecked_into();
            answers.insert(target.id().parse().unwrap_throw(), target.value());
            LocalStorage::set(target.id(), target.value()).unwrap_or_default();
        })
    };

    html! {
        <>
            { include_cdn() }
            <Container size={ContainerSize::ExtraSmall}>
                <Row class="vh-100 align-items-center">
                    <Column md={8} class="mx-auto">
                        <div class="text-center my-4">
                            <h1 class="mb-4">{ (*header).clone() }</h1>

                            {
                                if let Some(info) = (*alert_info).clone() {
                                    let text_vnode = Html::from_html_unchecked(AttrValue::from(info.text));
                                    html! { <Alert style={info.color} children={text_vnode} /> }
                                } else {
                                    html! { <></> }
                                }
                            }

                            <QuestionsList
                                questions={ questions.clone() }
                                oninput= { oninput.clone() }
                            />

                            <Button
                                class="w-100"
                                style={Color::Primary}
                                block={true}
                                size={ButtonSize::Large}
                                text="提交"
                                { onclick }
                            />
                        </div>
                    </Column>
                </Row>
            </Container>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

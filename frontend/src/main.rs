use std::collections::HashMap;

use yew::prelude::*;

use yew_bootstrap::component::card::*;
use yew_bootstrap::component::form::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

use yew_hooks::{use_map, use_title, UseMapHandle};

use gloo_net::http::Request;
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
    answers: UseMapHandle<usize, String>,
    onchange: Callback<Event>,
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
                let text = Html::from_html_unchecked(AttrValue::from(question.text.clone()));
                let hint = Html::from_html_unchecked(AttrValue::from(question.hint.clone()));
                html! {
                    <Card body=true class="mb-4 text-start">
                        <CardText class="mb-1">
                            <p>
                                <span>{ format!("{}. ", &question.id) }</span>
                                { text }
                                <b>{ format!("（{} 分）", &question.points) }</b>
                            </p>
                        </CardText>
                        <small class="text-muted">{ "提示：" }{ hint }</small>
                        <FormControl
                            id={ question.id.to_string() }
                            ctype={ FormControlType::Text }
                            class="my-2"
                            onchange={ props.onchange.clone() }
                            value={
                                props.answers.current()
                                    .get(&question.id)
                                    .cloned()
                                    .unwrap_or_default()
                            }
                        />
                    </Card>
                }
            }) }
        </>
    }
}

async fn get_info() -> Result<Info, String> {
    match Request::get("/api/info").send().await {
        Ok(response) => match response.json::<Info>().await {
            Ok(info) => Ok(info),
            Err(_) => Err("Failed to parse server response.".into()),
        },
        Err(_) => Err("Failed to get page information.".into()),
    }
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

    match Request::post("/api/answers").json(&answers_data) {
        Ok(request) => match request.send().await {
            Ok(response) => match response.json::<AnswersResponse>().await {
                Ok(result) => Ok(result),
                Err(_) => Err("Failed to parse server response.".into()),
            },
            Err(_) => Err("Failed to send answer.".into()),
        },
        Err(_) => Err("Failed to serialize JSON.".into()),
    }
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
            let header = header.clone();
            let questions = questions.clone();
            let answers = answers.clone();
            let alert_info = alert_info.clone();

            spawn_local(async move {
                match get_info().await {
                    Ok(info) => {
                        header.set(info.title);
                        info.questions.iter().for_each(|question| {
                            answers.insert(question.id, String::new());
                        });
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

    let onchange = {
        let answers = answers.clone();
        Callback::from(move |event: Event| {
            let target: HtmlInputElement = event.target_unchecked_into();
            answers.update(&target.id().parse().unwrap_throw(), target.value());
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
                                answers={ answers.clone() }
                                onchange={ onchange.clone() }
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
            { include_cdn_js() }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

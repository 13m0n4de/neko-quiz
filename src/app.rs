use std::collections::HashMap;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::api::*;
use crate::models::quiz::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="zh">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-gray-50">
                <App />
            </body>
        </html>
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AlertType {
    Success,
    Error,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
struct AlertInfo {
    alert_type: AlertType,
    text: String,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/neko-quiz.css" />
        <Title text="猫咪问答" />

        <Router>
            <main>
                <Routes fallback=|| "页面未找到".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
#[component]
pub fn Header(title: String) -> impl IntoView {
    view! {
        <header class="text-center mb-8">
            <h1 class="text-3xl font-semibold text-gray-900">{title}</h1>
        </header>
    }
}

#[component]
fn AlertDisplay(alert_info: ReadSignal<Option<AlertInfo>>) -> impl IntoView {
    view! {
        {move || {
            alert_info
                .get()
                .map(|info| {
                    let (bg_color, border_color, text_color) = match info.alert_type {
                        AlertType::Success => {
                            ("bg-teal-600/10", "border-teal-600/25", "text-teal-800")
                        }
                        AlertType::Error => ("bg-red-50", "border-red-200", "text-red-800"),
                        AlertType::Info => ("bg-gray-100", "border-gray-300", "text-gray-800"),
                    };

                    view! {
                        <div class=format!(
                            "whitespace-pre-line p-4 mb-6 rounded-md border text-center {} {} {}",
                            bg_color,
                            border_color,
                            text_color,
                        )>{info.text}</div>
                    }
                })
        }}
    }
}

#[component]
fn QuizForm(
    questions: Vec<Question>,
    submit: ServerAction<CreateSubmission>,
    answers: WriteSignal<HashMap<String, String>>,
) -> impl IntoView {
    view! {
        <ActionForm action=submit>
            <div class="questions-list space-y-6 mb-8">
                {questions
                    .into_iter()
                    .enumerate()
                    .map(|(idx, question)| {
                        view! {
                            <div class="question-card text-left bg-white rounded-md shadow-sm border border-gray-200 p-4">
                                <div class="question-header mb-3">
                                    <span class="text-teal-700 font-medium mr-2">
                                        {format!("{}.", idx + 1)}
                                    </span>
                                    <span class="text-gray-900" inner_html=question.text></span>
                                    <span class="text-teal-700 ml-2">
                                        {format!("（{} 分）", question.points)}
                                    </span>
                                </div>
                                <div class="question-hint mb-4">
                                    <span
                                        class="text-sm text-gray-500"
                                        inner_html=question.hint
                                    ></span>
                                </div>
                                <div class="answer-input">
                                    <input
                                        type="text"
                                        name=format!("answers[{}]", question.id)
                                        class="w-full px-3 py-2 border border-gray-300 rounded-md
                                        focus:outline-none focus:border-2 focus:border-teal-500
                                        bg-white text-gray-900"
                                        placeholder="请输入您的答案"
                                        on:input:target=move |ev| {
                                            answers
                                                .write()
                                                .insert(ev.target().name(), ev.target().value());
                                        }
                                    />
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="submit-section text-center">
                <input
                    type="submit"
                    value="提交答案"
                    class="w-full bg-teal-600 hover:bg-teal-700 text-white font-medium
                    py-3 px-12 rounded-md transition-colors duration-200
                    focus:outline-none focus:ring-2 focus:ring-teal-500 focus:ring-offset-2
                    disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled=move || submit.pending().get()
                />
            </div>
        </ActionForm>
    }
}

#[component]
fn QuizContent(quiz_data: Quiz) -> impl IntoView {
    let alert_info = RwSignal::new(Option::<AlertInfo>::None);
    let answers = RwSignal::new(HashMap::<String, String>::new());
    let submit = ServerAction::<CreateSubmission>::new();

    Effect::new(move |_| {
        if let Some(result) = submit.value().get() {
            match result {
                Ok(response) => {
                    let alert_type = if response.status {
                        AlertType::Success
                    } else {
                        AlertType::Info
                    };
                    let score_text = response
                        .score
                        .map(|s| format!("本次测验总得分为 {s}。\n"))
                        .unwrap_or_default();
                    alert_info.set(Some(AlertInfo {
                        alert_type,
                        text: format!("{score_text}{}", response.message),
                    }));
                }
                Err(err) => {
                    alert_info.set(Some(AlertInfo {
                        alert_type: AlertType::Error,
                        text: format!("提交失败: {err}"),
                    }));
                }
            }
        }
    });

    view! {
        <div class="quiz-container">
            <Header title=quiz_data.title />
            <AlertDisplay alert_info=alert_info.read_only() />
            <QuizForm questions=quiz_data.questions submit=submit answers=answers.write_only() />
        </div>
    }
}

#[component]
fn ErrorView(error_message: String, on_retry: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div>
            <div class=format!(
                "p-4 mb-6 rounded-md border text-center {} {} {}",
                "bg-red-50",
                "border-red-200",
                "text-red-800",
            )>{error_message}</div>

            <div class="text-center">
                <button
                    on:click=move |_| on_retry()
                    class="w-full bg-teal-600 hover:bg-teal-700 text-white font-medium
                    py-2 px-6 rounded-md transition-colors duration-200
                    focus:outline-none focus:ring-2 focus:ring-teal-500 focus:ring-offset-2"
                >
                    "重新加载"
                </button>
            </div>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let quiz_resource = Resource::new(|| (), |_| async { get_quiz().await });

    view! {
        <div class="min-h-screen bg-gray-50 py-12">
            <div class="max-w-4xl mx-auto px-4">
                <Suspense fallback=|| {
                    view! {
                        <div class="text-center">
                            <p class="text-gray-500">"正在加载题目..."</p>
                        </div>
                    }
                }>
                    {move || {
                        quiz_resource
                            .get()
                            .map(|result| match result {
                                Ok(quiz_data) => view! { <QuizContent quiz_data /> }.into_any(),
                                Err(err) => {
                                    view! {
                                        <ErrorView
                                            error_message=format!("加载题目失败: {}", err)
                                            on_retry=move || quiz_resource.refetch()
                                        />
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

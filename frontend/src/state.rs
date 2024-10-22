use gloo_storage::{LocalStorage, Storage};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::{create_submission, get_quiz};
use crate::models::{AlertInfo, AlertType, Question, Quiz, QuizResponse};

const VERSION_KEY: &str = "quiz_version";
const ANSWERS_KEY: &str = "quiz_answers";

#[derive(Default, PartialEq, Clone)]
pub struct AppState {
    pub header: String,
    pub questions: Rc<Vec<Question>>,
    pub answers: Rc<RefCell<HashMap<String, String>>>,
    pub alert_info: Option<AlertInfo>,
}

impl AppState {
    fn get_stored_version() -> u64 {
        LocalStorage::get(VERSION_KEY).unwrap_or_default()
    }

    fn set_stored_version(version: u64) {
        LocalStorage::set(VERSION_KEY, version).expect("Failed to set quiz version");
    }

    fn get_stored_answers() -> HashMap<String, String> {
        LocalStorage::get(ANSWERS_KEY).unwrap_or_default()
    }

    fn set_stored_answers(answers: &HashMap<String, String>) {
        LocalStorage::set(ANSWERS_KEY, answers).expect("Failed to save answers to local storage");
    }

    fn clear_local_storage() {
        LocalStorage::clear();
    }
}

pub enum AppAction {
    SetQuiz(Quiz),
    SetAnswer(String, String),
    SetAlertInfo(Option<AlertInfo>),
    SetQuizResponse(QuizResponse),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppAction::SetQuiz(quiz) => {
                let new_version = quiz.version;
                let stored_version = Self::get_stored_version();

                let answers = if new_version == stored_version {
                    Self::get_stored_answers()
                } else {
                    Self::clear_local_storage();
                    Self::set_stored_version(new_version);
                    HashMap::new()
                };

                Rc::new(AppState {
                    header: quiz.title,
                    questions: Rc::new(quiz.questions),
                    answers: Rc::new(RefCell::new(answers)),
                    ..(*self).clone()
                })
            }
            AppAction::SetAnswer(id, answer) => {
                self.answers.borrow_mut().insert(id.clone(), answer.clone());
                self
            }
            AppAction::SetAlertInfo(info) => Rc::new(AppState {
                alert_info: info,
                ..(*self).clone()
            }),
            AppAction::SetQuizResponse(response) => {
                let alert_type = if response.status {
                    AlertType::Success
                } else {
                    AlertType::Info
                };
                let score_text = response
                    .score
                    .map(|s| format!("本次测验总得分为 {s}。<br/>"))
                    .unwrap_or_default();
                Rc::new(AppState {
                    alert_info: Some(AlertInfo {
                        alert_type,
                        text: format!("{score_text}{}", response.message),
                    }),
                    ..(*self).clone()
                })
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct AppContext {
    pub state: UseReducerHandle<AppState>,
}

impl AppContext {
    pub fn new(state: UseReducerHandle<AppState>) -> Self {
        Self { state }
    }

    pub fn get_quiz(&self) {
        let state = self.state.clone();
        spawn_local(async move {
            match get_quiz().await {
                Ok(quiz) => {
                    state.dispatch(AppAction::SetQuiz(quiz));
                }
                Err(err) => state.dispatch(AppAction::SetAlertInfo(Some(AlertInfo {
                    alert_type: AlertType::Error,
                    text: err.to_string(),
                }))),
            }
        });
    }

    pub fn create_submission(&self) {
        let state = self.state.clone();
        let answers = state.answers.borrow().clone();
        AppState::set_stored_answers(&answers);
        spawn_local(async move {
            match create_submission(answers).await {
                Ok(response) => state.dispatch(AppAction::SetQuizResponse(response)),
                Err(err) => state.dispatch(AppAction::SetAlertInfo(Some(AlertInfo {
                    alert_type: AlertType::Error,
                    text: err.to_string(),
                }))),
            }
        });
    }
}

#[hook]
pub fn use_app_context() -> AppContext {
    use_context::<AppContext>().expect("No context found!")
}

use gloo_storage::{LocalStorage, Storage};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_bootstrap::util::Color;

use crate::api::{get_info, submit_answers};
use crate::models::{AlertInfo, Question, Quiz, QuizResponse};

const VERSION_KEY: &str = "quiz_version";

#[derive(Default, PartialEq, Clone)]
pub struct AppState {
    pub header: String,
    pub questions: Rc<Vec<Question>>,
    pub answers: Rc<RefCell<HashMap<String, String>>>,
    pub alert_info: Option<AlertInfo>,
}

impl AppState {
    fn get_stored_version() -> u64 {
        LocalStorage::get(VERSION_KEY).unwrap_or(0)
    }

    fn set_stored_version(version: u64) {
        LocalStorage::set(VERSION_KEY, version).expect("Failed to set quiz version");
    }

    fn clear_local_storage() {
        LocalStorage::clear();
    }
}

pub enum AppAction {
    SetInfo(Quiz),
    SetAnswer(String, String),
    SetAlertInfo(Option<AlertInfo>),
    LoadStoredAnswers,
    SetQuizResponse(QuizResponse),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppAction::SetInfo(info) => {
                let new_version = info.version;
                let stored_version = Self::get_stored_version();

                let should_clear_storage = stored_version != new_version;

                if should_clear_storage {
                    Self::clear_local_storage();
                    Self::set_stored_version(new_version);
                }

                Rc::new(AppState {
                    header: info.title,
                    questions: Rc::new(info.questions),
                    answers: if should_clear_storage {
                        Rc::new(RefCell::new(HashMap::new()))
                    } else {
                        self.answers.clone()
                    },
                    ..(*self).clone()
                })
            }
            AppAction::SetAnswer(id, answer) => {
                self.answers.borrow_mut().insert(id.clone(), answer.clone());
                LocalStorage::set(id, answer).expect("Failed to set answer in local storage");
                self
            }
            AppAction::SetAlertInfo(info) => Rc::new(AppState {
                alert_info: info,
                ..(*self).clone()
            }),
            AppAction::LoadStoredAnswers => {
                let stored_answers = self
                    .questions
                    .iter()
                    .filter_map(|q| {
                        LocalStorage::get(&q.id)
                            .ok()
                            .map(|answer| (q.id.clone(), answer))
                    })
                    .collect();
                Rc::new(AppState {
                    answers: Rc::new(RefCell::new(stored_answers)),
                    ..(*self).clone()
                })
            }
            AppAction::SetQuizResponse(response) => {
                let color = if response.status {
                    Color::Success
                } else {
                    Color::Secondary
                };
                Rc::new(AppState {
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
pub struct AppContext {
    pub state: UseReducerHandle<AppState>,
}

impl AppContext {
    pub fn new(state: UseReducerHandle<AppState>) -> Self {
        Self { state }
    }

    pub fn fetch_info(&self) {
        let state = self.state.clone();
        spawn_local(async move {
            match get_info().await {
                Ok(info) => {
                    state.dispatch(AppAction::SetInfo(info));
                    state.dispatch(AppAction::LoadStoredAnswers);
                }
                Err(err) => state.dispatch(AppAction::SetAlertInfo(Some(AlertInfo {
                    color: Color::Danger,
                    text: err.to_string(),
                }))),
            }
        });
    }

    pub fn submit_answers(&self) {
        let state = self.state.clone();
        let answers = state.answers.borrow().clone();
        spawn_local(async move {
            match submit_answers(answers).await {
                Ok(response) => state.dispatch(AppAction::SetQuizResponse(response)),
                Err(err) => state.dispatch(AppAction::SetAlertInfo(Some(AlertInfo {
                    color: Color::Danger,
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

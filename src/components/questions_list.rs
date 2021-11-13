use serde::Deserialize;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Question {
    pub id: usize,
    pub question: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct QuestionsListProps {
    pub questions: Vec<Question>,
    pub on_click: Callback<Question>
}

#[function_component(QuestionsList)]
pub fn questions_list(QuestionsListProps { questions, on_click }: &QuestionsListProps) -> Html {
    questions.iter().map(|question| {
      let on_question_request = {
        let on_click = on_click.clone();
        let question = question.clone();
        Callback::from(move |_| {
            on_click.emit(question.clone())
        })
      };

      html! {
        <p onclick={on_question_request}>{format!("{}", question.question)}</p>
    }
  }).collect()
}
use crate::error::Error;
use crate::types::tasks::GetAllTasks;
use gloo::console;
use serde::de::DeserializeOwned;
use yew::{function_component, html, use_effect_with_deps};
use yew_hooks::use_async;

#[function_component(EventTable)]
pub fn event_table() -> Html {
    let state = use_async(async move { fetch_all_tasks().await });

    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                state.run();
                || ()
            },
            (),
        );
    }

    if let Some(data) = &state.data {
        // data;
        // data.hello; // it knows hello doesnt exist 'no field `hello` on type `&GetAllTasks`'
        if !data.results.is_empty() {
            html! {
                <>
                // { data.results[0].hello } // it knows hello doesnt exist 'no field `hello` on type `&GetAllTasks`'
                  {for data.results[0].tasks.iter().map(|tasks| {
                      html! {
                        <p class="task-name">{tasks.name.to_string()}</p>
                      }
                    })
                  }
                </>
            }
        } else {
            html! {
                <div class="task-preview">{ "No tasks..." }</div>
            }
        }
    } else {
        html! {
            <div class="task-preview">{ "Loading..." }</div>
        }
    }
}

async fn fetch_all_tasks() -> Result<GetAllTasks, Error> {
    fetch::<GetAllTasks>(format!("http://localhost:5000/tasks/group?page=1")).await
}

async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Bad error handling and not prod ready, just a quick prototype
    let response = reqwest::get(url).await.unwrap();
    let text = response.text().await.unwrap();
    let v: T = serde_json::from_str(&text).unwrap();

    Ok(v)
}

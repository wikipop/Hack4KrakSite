use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned background image", body = String),
        (status = 400, description = "Invalid task id."),
        (status = 404, description = "Task not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "tasks"
)]
#[get("/background/{task_id}")]
pub async fn background(
    app_state: Data<AppState>,
    request: HttpRequest,
    task_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let manager = &app_state.task_manager;

    let content = manager
        .load_asset(&task_id, "pictures/background.png")
        .await?;

    Ok(content.into_response(&request))
}

use jelly::actix_web::{HttpRequest, HttpResponse};
use jelly::prelude::*;
use jelly::Result;
use crate::qa::models::Question;

pub async fn index(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;

    let questons = Question::find_all(db).await?;
    request.render(200, "qa/index.html", {
        let mut context = Context::new();
        context.insert("questions", &questons);
        context
    })

}
use jelly::actix_web::{HttpRequest, HttpResponse, web};
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


pub async fn question_detail(request: HttpRequest) -> Result<HttpResponse> {
    let db = request.db_pool()?;
    let id1 = request.match_info().get("id").unwrap().parse().unwrap();
    let id2 :i32= request.match_info().query("id").parse().unwrap();
    let q = Question::get_by_id(id1, db).await?;

    request.render(200, "qa/detail.html", {
        let mut context = Context::new();
        context.insert("q", &q);
        context
    })
}
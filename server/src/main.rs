use actix_web::{get, guard, web, App, HttpServer, Responder};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod graphql;

async fn graphql(schema: todo!(), req: GraphQLRequest) -> GraphQLResponse {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(todo!(), EmptyMutation, EmptySubscription).finish();

    HttpServer::new(|| {
        App::new().service(web::resource("/graphql").guard(guard::Post()).to(graphql))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}

use async_graphql::Result;

pub struct Queries;

#[Object]
impl Queries {
    async fn hello(&self, name: Option<String>) -> Result<String> {
        // todo
    }
}

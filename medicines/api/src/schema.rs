use juniper::{FieldResult, RootNode};

use crate::{
    azure_search::AzureContext,
    product::get_substance_with_products,
    substance::{get_substances, Substance, Substances},
};

pub struct QueryRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl QueryRoot {
    async fn substance(context: &AzureContext, name: Option<String>) -> FieldResult<Substance> {
        if let Some(name) = name {
            return Ok(get_substance_with_products(name, &context.client).await);
        }

        Err(juniper::FieldError::new(
            "Getting a list of products without providing a substance name is not currently supported.",
            juniper::Value::null()
        ))
    }

    async fn substances(first: i32) -> FieldResult<Substances> {
        Ok(get_substances(first).await)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

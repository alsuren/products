use juniper::{FieldResult, RootNode};

use crate::{
    azure_context::AzureContext,
    product::get_substance_with_products,
    substance::{get_substances, get_substances_starting_with_letter, Substance, Substances},
};

pub struct QueryRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl QueryRoot {
    async fn substance(context: &AzureContext, name: Option<String>) -> FieldResult<Substance> {
        match name {
            Some(name) => get_substance_with_products(&name, &context.client)
                .await
                .map_err(|e| {
                    tracing::error!("Error fetching results from Azure search service: {:?}", e);
                    juniper::FieldError::new(
                        "Error fetching search results",
                        juniper::Value::null(),
                    )
                }),
            None => Err(juniper::FieldError::new(
                "Getting a substance without providing a substance name is not supported.",
                juniper::Value::null(),
            )),
        }
    }

    async fn substances_by_first_letter(
        context: &AzureContext,
        letter: String,
    ) -> FieldResult<Substances> {
        get_substances_starting_with_letter(&context.client, letter.chars().next().unwrap())
            .await
            .map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                juniper::FieldError::new("Error fetching search results", juniper::Value::null())
            })
    }

    async fn substances(first: i32) -> FieldResult<Substances> {
        Ok(get_substances(first).await)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct SubscriptionRoot;

#[juniper::graphql_subscription(Context = AzureContext)]
impl SubscriptionRoot {}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, SubscriptionRoot {})
}

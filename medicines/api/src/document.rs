use crate::{pagination, pagination::PageInfo};
use juniper::GraphQLObject;
use search_client::{models::IndexResult, AzureFieldFilter, AzureFilterSet, Search};

#[derive(GraphQLObject, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[graphql(description = "A document")]
pub struct Document {
    product_name: Option<String>,
    active_substances: Option<Vec<String>>,
    title: Option<String>,
    highlights: Option<Vec<String>>,
    created: Option<String>,
    doc_type: Option<String>,
    file_size_in_bytes: Option<i32>,
    name: Option<String>,
    url: Option<String>,
}

impl From<IndexResult> for Document {
    fn from(r: IndexResult) -> Self {
        Self {
            product_name: r.product_name,
            active_substances: Some(r.substance_name),
            title: Some(r.title),
            created: r.created,
            doc_type: Some(r.doc_type),
            file_size_in_bytes: Some(r.metadata_storage_size),
            name: Some(r.file_name),
            url: Some(r.metadata_storage_path),
            highlights: match r.highlights {
                Some(a) => Some(a.content),
                _ => None,
            },
        }
    }
}

pagination! {Documents, DocumentEdge, Document}

fn get_document_edges(docs: Vec<Document>, offset: i32) -> Vec<DocumentEdge> {
    docs.into_iter()
        .enumerate()
        .map(|(i, document)| DocumentEdge {
            node: document,
            cursor: base64::encode((i as i32 + offset).to_string()),
        })
        .collect()
}

fn get_documents_from_edges(edges: Vec<DocumentEdge>, offset: i32, total_count: i32) -> Documents {
    let result_count = edges.len() as i32;
    let has_previous_page = offset != 0;
    let has_next_page = offset + result_count < total_count;
    let start_cursor = base64::encode(offset.to_string());
    let end_cursor =
        base64::encode(std::cmp::min(total_count, offset + result_count - 1).to_string());

    Documents {
        edges,
        total_count,
        page_info: PageInfo {
            has_previous_page,
            has_next_page,
            start_cursor,
            end_cursor,
        },
    }
}

pub fn get_documents_graph_from_documents_vector(
    docs: Vec<Document>,
    offset: i32,
    total_count: i32,
) -> Documents {
    let edges = get_document_edges(docs, offset);
    get_documents_from_edges(edges, offset, total_count)
}

pub async fn get_documents(
    client: &impl Search,
    search: String,
    first: Option<i32>,
    after: Option<String>,
    document_types: Option<Vec<String>>,
) -> Result<Documents, anyhow::Error> {
    let result_count = first.unwrap_or(10);
    let offset = match after {
        Some(after) => {
            let bytes = base64::decode(after)?;
            let string = std::str::from_utf8(&bytes)?;
            string.parse::<i32>()? + 1
        }
        None => 0,
    };
    let filter_set = build_document_types_filter_set(document_types);

    let azure_result = client
        .search_with_pagination_and_filter(
            &search,
            search_client::AzurePagination {
                result_count,
                offset,
            },
            true,
            filter_set,
        )
        .await?;

    let docs = azure_result
        .search_results
        .iter()
        .map(|search_result| Document::from(search_result.clone()))
        .collect();

    let total_count = azure_result.count.unwrap_or(0);

    Ok(get_documents_graph_from_documents_vector(
        docs,
        offset,
        total_count,
    ))
}

fn build_document_types_filter_set(document_types: Option<Vec<String>>) -> AzureFilterSet {
    match document_types {
        Some(document_types) => AzureFilterSet {
            boolean_operator: "or".to_string(),
            field_filters: document_types
                .into_iter()
                .map(|document_type| AzureFieldFilter {
                    field_name: "doc_type".to_string(),
                    operator: "eq".to_string(),
                    field_value: format!(
                        "{}{}",
                        document_type[..1].to_uppercase(),
                        document_type[1..].to_lowercase()
                    ),
                })
                .collect(),
        },
        None => AzureFilterSet {
            boolean_operator: "or".to_string(),
            field_filters: vec![],
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async_trait::async_trait;
    use search_client::{models::IndexResults, AzureFilterSet};
    use tokio_test::block_on;

    struct TestAzureSearchClient {
        pub search_results: Vec<IndexResult>,
    }

    impl TestAzureSearchClient {
        fn new(search_results: Vec<IndexResult>) -> Self {
            Self { search_results }
        }
    }

    #[async_trait]
    impl Search for TestAzureSearchClient {
        async fn search(&self, _search_term: &str) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn search_with_pagination(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!();
        }
        async fn search_with_pagination_and_filter(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
            _filter: search_client::AzureFilterSet,
        ) -> Result<IndexResults, reqwest::Error> {
            Ok(IndexResults {
                search_results: self.search_results.clone(),
                context: String::from(""),
                count: Some(1234),
            })
        }
        async fn filter_by_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn filter_by_non_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
    }

    fn given_a_search_result(product_name: &str) -> IndexResult {
        IndexResult {
            product_name: Some(product_name.to_string()),
            doc_type: "Spc".to_string(),
            file_name: "our_id".to_string(),
            metadata_storage_name: "storage_name".to_string(),
            metadata_storage_path: "test/path".to_string(),
            substance_name: vec!["substance".to_string()],
            title: "title".to_string(),
            created: None,
            facets: vec!["facet".to_string()],
            keywords: None,
            metadata_storage_size: 300,
            release_state: None,
            rev_label: None,
            suggestions: vec!["suggestion".to_string()],
            score: 1.0,
            highlights: None,
        }
    }

    fn given_first_page_of_search_results() -> Vec<IndexResult> {
        vec![
            given_a_search_result("first"),
            given_a_search_result("second"),
            given_a_search_result("third"),
            given_a_search_result("fourth"),
            given_a_search_result("fifth"),
            given_a_search_result("sixth"),
            given_a_search_result("seventh"),
            given_a_search_result("eighth"),
            given_a_search_result("ninth"),
            given_a_search_result("tenth"),
        ]
    }

    fn given_last_page_of_search_results() -> Vec<IndexResult> {
        vec![
            given_a_search_result("fourth last"),
            given_a_search_result("third last"),
            given_a_search_result("second last"),
            given_a_search_result("last"),
        ]
    }

    fn given_a_search_client(search_results: &Vec<IndexResult>) -> TestAzureSearchClient {
        TestAzureSearchClient::new(search_results.clone())
    }

    fn when_we_get_the_first_page_of_documents(search_client: impl Search) -> Documents {
        block_on(get_documents(
            &search_client,
            "Search string".to_string(),
            None,
            None,
            None,
        ))
        .unwrap()
    }

    fn when_we_get_the_last_page_of_documents(search_client: impl Search) -> Documents {
        block_on(get_documents(
            &search_client,
            "Search string".to_string(),
            None,
            Some(base64::encode("1229").to_string()),
            None,
        ))
        .unwrap()
    }

    fn then_we_have_the_first_page(documents_response: &Documents) {
        let expected_names = vec![
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth",
        ];
        let edges = &documents_response.edges;
        let actual_names = edges
            .into_iter()
            .map(|edge| edge.node.product_name.as_ref().unwrap());
        assert!(actual_names.eq(expected_names));

        assert_eq!(1234, documents_response.total_count);

        let expected_page_info = PageInfo {
            has_previous_page: false,
            has_next_page: true,
            start_cursor: base64::encode("0"),
            end_cursor: base64::encode("9"),
        };
        assert_eq!(expected_page_info, documents_response.page_info);
    }

    fn then_we_have_the_last_page(documents_response: &Documents) {
        let expected_names = vec!["fourth last", "third last", "second last", "last"];
        let edges = &documents_response.edges;
        let actual_names = edges
            .into_iter()
            .map(|edge| edge.node.product_name.as_ref().unwrap());

        assert!(actual_names.eq(expected_names));

        assert_eq!(1234, documents_response.total_count);
        let expected_page_info = PageInfo {
            has_previous_page: true,
            has_next_page: false,
            start_cursor: base64::encode("1230"),
            end_cursor: base64::encode("1233"),
        };
        assert_eq!(expected_page_info, documents_response.page_info);
    }

    #[test]
    fn test_get_documents_first_page() {
        let search_results = given_first_page_of_search_results();
        let search_client = given_a_search_client(&search_results);
        let response = when_we_get_the_first_page_of_documents(search_client);
        then_we_have_the_first_page(&response);
    }

    #[test]
    fn test_get_documents_last_page() {
        let search_results = given_last_page_of_search_results();
        let search_client = given_a_search_client(&search_results);
        let response = when_we_get_the_last_page_of_documents(search_client);
        then_we_have_the_last_page(&response);
    }

    #[test]
    fn test_build_document_types_filter_set() {
        let document_types = Some(vec![
            "SPC".to_string(),
            "pil".to_string(),
            "PaR".to_string(),
        ]);
        let expected_filter_set = AzureFilterSet {
            boolean_operator: "or".to_string(),
            field_filters: vec![
                AzureFieldFilter {
                    field_name: "doc_type".to_string(),
                    operator: "eq".to_string(),
                    field_value: "Spc".to_string(),
                },
                AzureFieldFilter {
                    field_name: "doc_type".to_string(),
                    operator: "eq".to_string(),
                    field_value: "Pil".to_string(),
                },
                AzureFieldFilter {
                    field_name: "doc_type".to_string(),
                    operator: "eq".to_string(),
                    field_value: "Par".to_string(),
                },
            ],
        };
        let actual_filter_set = build_document_types_filter_set(document_types);
        assert_eq!(expected_filter_set, actual_filter_set);
    }

    #[test]
    fn test_build_empty_document_types_filter_set() {
        let document_types = None;
        let expected_filter_set = AzureFilterSet {
            boolean_operator: "or".to_string(),
            field_filters: vec![],
        };
        let actual_filter_set = build_document_types_filter_set(document_types);
        assert_eq!(expected_filter_set, actual_filter_set);
    }
}

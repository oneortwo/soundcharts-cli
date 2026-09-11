use crate::{
    cli::PaginationArgs,
    client::SoundchartsClient,
    output::{self, OutputFormat},
    paginator,
};

pub async fn get(client: &SoundchartsClient, uuid: &str, format: &OutputFormat) {
    let response = client
        .get(&format!("/api/v2/label/{}", urlencoding::encode(uuid)), &[])
        .await;
    let object = &response.body["object"];
    if *format == OutputFormat::Json {
        output::print_json(object);
    } else {
        output::print_records(
            std::slice::from_ref(object),
            &[
                "name",
                "uuid",
                "type",
                "countryCode",
                "foundedAt",
                "website",
                "company",
                "address",
            ],
            format,
        );
    }
}

pub async fn identifiers(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/label/{}/identifiers", urlencoding::encode(uuid));
    let result = paginator::paginate(client, &path, &[], pagination).await;
    output::print_records(
        &result.items,
        &["platformName", "platformCode", "identifier", "url"],
        format,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{pagination, server};
    use serde_json::json;

    #[tokio::test]
    async fn identifiers_follow_pages() {
        let (client, requests) = server(vec![
            json!({"items":[{"identifier":"one"}],"page":{"next":"next"}}),
            json!({"items":[],"page":{"next":null}}),
        ])
        .await;
        identifiers(&client, "id", &pagination(), &OutputFormat::Json).await;
        let requests = requests.await.unwrap();
        assert_eq!(requests[0].path(), "/api/v2/label/id/identifiers");

        assert!(requests[1]
            .query_pairs()
            .any(|(k, v)| k == "offset" && v == "100"));
    }

    #[tokio::test]
    async fn label_metadata_uses_uuid() {
        let (client, requests) = server(vec![json!({"object":{"uuid":"id","name":"Label"}})]).await;
        get(&client, "id", &OutputFormat::Json).await;
        assert_eq!(requests.await.unwrap()[0].path(), "/api/v2/label/id");
    }
}

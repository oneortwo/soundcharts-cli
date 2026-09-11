use crate::{
    client::SoundchartsClient,
    output::{self, OutputFormat},
};

pub async fn run(client: &SoundchartsClient, format: &OutputFormat) {
    let response = client.get("/api/v2/team/usage", &[]).await;
    if *format == OutputFormat::Json {
        output::print_json(&response.body);
    } else {
        let rows: Vec<_> = response
            .body
            .as_object()
            .into_iter()
            .flat_map(|object| object.iter())
            .map(|(key, value)| serde_json::json!({"metric":key,"value":value}))
            .collect();
        output::print_records(&rows, &["metric", "value"], format);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::server;
    use serde_json::json;

    #[tokio::test]
    async fn quota_reads_team_usage_without_query_parameters() {
        let (client, requests) = server(vec![json!({"quota":10})]).await;
        run(&client, &OutputFormat::Json).await;
        let requests = requests.await.unwrap();
        assert_eq!(requests[0].path(), "/api/v2/team/usage");
        assert_eq!(requests[0].query(), None);
    }
}

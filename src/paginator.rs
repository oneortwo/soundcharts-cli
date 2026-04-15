use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use console::Term;
use serde_json::Value;

pub struct PaginateResult {
    pub items: Vec<Value>,
    pub total: Option<u64>,
    pub quota_remaining: Option<u64>,
}

pub async fn paginate(
    client: &SoundchartsClient,
    path: &str,
    base_params: &[(&str, String)],
    args: &PaginationArgs,
) -> PaginateResult {
    let is_tty = Term::stdout().is_term();
    let mut all_items: Vec<Value> = Vec::new();
    let mut offset: usize = 0;
    let mut total: Option<u64> = None;
    let mut quota_remaining: Option<u64> = None;

    let max_items = if args.all {
        None
    } else if let Some(limit) = args.limit {
        Some(limit)
    } else if args.no_paginate {
        Some(args.page_size)
    } else {
        Some(args.page_size)
    };

    loop {
        let mut params: Vec<(&str, String)> = base_params.to_vec();
        params.push(("offset", offset.to_string()));
        params.push(("limit", args.page_size.to_string()));

        let param_refs: Vec<(&str, &str)> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let response = client.get(path, &param_refs).await;
        quota_remaining = response.quota_remaining;

        let body = &response.body;

        if total.is_none() {
            total = body.pointer("/page/total").and_then(|t| t.as_u64());
        }

        let items = match body.get("items").and_then(|i| i.as_array()) {
            Some(items) => items.clone(),
            None => break,
        };

        if items.is_empty() {
            break;
        }

        all_items.extend(items.iter().cloned());

        if is_tty && (args.all || args.limit.is_some()) {
            if let Some(t) = total {
                eprint!("\rFetching... {}/{} items", all_items.len(), t);
            } else {
                eprint!("\rFetching... {} items", all_items.len());
            }
        }

        if let Some(max) = max_items {
            if all_items.len() >= max {
                all_items.truncate(max);
                break;
            }
        }

        let has_next = body
            .pointer("/page/next")
            .map(|n| !n.is_null())
            .unwrap_or(false);
        if !has_next {
            break;
        }

        offset += args.page_size;

        if !args.all && args.limit.is_none() {
            break;
        }
    }

    if is_tty && (args.all || args.limit.is_some()) && !all_items.is_empty() {
        eprintln!();
    }

    PaginateResult {
        items: all_items,
        total,
        quota_remaining,
    }
}

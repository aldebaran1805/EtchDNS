use extism_pdk::*;
use serde::{Deserialize, Serialize};

/// Input structure for the hook_client_query_received function
#[derive(Deserialize)]
struct QueryInput {
    query_name: String,
    qtype: u16,
    qclass: u16,
    client_ip: String,
}

/// Hook function called when a client query is received
///
/// # Arguments
///
/// * Input JSON: {"query_name": "example.com", "qtype": 1, "qclass": 1, "client_ip": "192.168.1.1"}
///
/// # Returns
///
/// An integer code:
/// * 0 - Continue normal processing
/// * -1 - Return a minimal response with REFUSED rcode
#[plugin_fn]
pub fn hook_client_query_received(input: String) -> FnResult<String> {
    // Parse the input JSON
    let query_input: QueryInput = serde_json::from_str(&input)?;

    // Log the query details
    info!(
        "Received query for {} (type: {}, class: {}) from client IP {}",
        query_input.query_name, query_input.qtype, query_input.qclass, query_input.client_ip
    );

    // For demonstration purposes, let's implement a simple rule:
    // If the query is for "example.com" OR the client IP is "192.168.1.100", return -1 to refuse the query
    // Otherwise, return 0 to continue normal processing
    let result =
        if query_input.query_name == "example.com" || query_input.client_ip == "192.168.1.100" {
            -1
        } else {
            0
        };

    // Return the result as a string
    Ok(result.to_string())
}

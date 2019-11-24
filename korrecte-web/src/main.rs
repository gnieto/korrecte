use anyhow::{anyhow, Context, Result};
use gotham::helpers::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use korrecte::executor::{ExecutionContextBuilder, ExecutionMode, Executor};
use korrecte::reporting::{Finding, Reporter};
use std::path::Path;

fn main() {
    let addr = "0.0.0.0:8000";
    gotham::start(addr, router())
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/evaluate").to(evaluator_handler);
        route.get("/ping").to(ping_handler);
    })
}

pub fn ping_handler(state: State) -> (State, String) {
    (state, "ok".to_string())
}

pub fn evaluator_handler(state: State) -> (State, Response<Body>) {
    let result = inner_evaluator_handler();

    match result {
        Ok(findings) => ok_response(state, findings),
        Err(e) => error_response(state, e),
    }
}

fn inner_evaluator_handler() -> Result<String, anyhow::Error> {
    let findings = analyze_cluster()?;
    let findings_as_str = serde_json::to_string(&findings).context("Could not encode findings")?;

    Ok(findings_as_str)
}

fn analyze_cluster() -> Result<Vec<Finding>, anyhow::Error> {
    let context = ExecutionContextBuilder::default()
        .configuration_from_path(Path::new("korrecte.toml"))
        .map_err(|e| anyhow!("Could not find configuration file: {:?}", e))?
        .execution_mode(ExecutionMode::Api)
        .build();

    let executor = Executor::with_context(context);

    executor
        .execute()
        .map(|reporter| reporter.findings())
        .map_err(|e| anyhow!("Errored while linting: {:?}", e))
}

fn ok_response(state: State, findings: String) -> (State, Response<Body>) {
    let response = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, findings);

    (state, response)
}

fn error_response(state: State, e: anyhow::Error) -> (State, Response<Body>) {
    let response = create_response(
        &state,
        StatusCode::INTERNAL_SERVER_ERROR,
        mime::TEXT_PLAIN,
        format!("{:?}", e),
    );

    (state, response)
}

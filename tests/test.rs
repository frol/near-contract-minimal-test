const WASM_FILEPATH: &str = "./contract.wasm";

#[tokio::test]
async fn test_hello_world() {
    let worker = near_workspaces::sandbox().await.expect("SANDBOX FAILED");
    let wasm = std::fs::read(WASM_FILEPATH).expect("READ contract.wasm WASM FILE FAILED");
    let contract = worker.dev_deploy(&wasm).await.expect("DEPLOY FAILED");

    let outcome = contract
        .call("hello_world")
        .args_json(serde_json::json!({
                "name": contract.id(),
        }))
        .transact()
        .await
        .expect("CALL FAILED");

    println!("hello world outcome: {:#?}", outcome);
}

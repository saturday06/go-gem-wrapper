use futures::future::join_all;
use magnus::{Error, Ruby, function};
use std::sync::{LazyLock, Mutex};

fn tarai(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        y
    } else {
        tarai(tarai(x - 1, y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))
    }
}

async fn tarai_tokio_async(x: i32, y: i32, z: i32, times: i32) -> Vec<i32> {
    let tasks = (0..times).map(|_| tokio::task::spawn(async move { tarai(x, y, z) }));
    let result_and_errors = join_all(tasks).await;
    let results = result_and_errors
        .iter()
        .map(|result| *result.as_ref().expect("Taskのジョインに失敗しました"))
        .collect();
    results
}

static RUNTIME: LazyLock<Mutex<tokio::runtime::Runtime>> = LazyLock::new(|| {
    Mutex::new(tokio::runtime::Runtime::new().expect("TokioのRuntimeの作成に失敗しました"))
});

fn tarai_tokio(x: i32, y: i32, z: i32, times: i32) -> Vec<i32> {
    let runtime = RUNTIME.lock().expect("Tokio Runtimeの取得に失敗しました");
    runtime.block_on(tarai_tokio_async(x, y, z, times))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    ruby.define_global_function("tarai_rust", function!(tarai, 3));
    ruby.define_global_function("tarai_rust_tokio", function!(tarai_tokio, 4));
    return Ok(());
}

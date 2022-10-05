use std::thread;

use aquadoggo::{Configuration, Node};

#[cfg(target_os = "android")]
#[no_mangle]
unsafe extern "C" fn Java_com_example_doggodroid_MainActivity_startDoggo() {
    // @TODO: Replace this with `android_logger` directly
    ndk_glue::android_logger::init_once(
        ndk_glue::android_logger::Config::default().with_min_level(ndk_glue::log::Level::Debug),
    );

    thread::spawn(move || main());
}

#[tokio::main]
pub async fn main() {
    let config = Configuration {
        // @TODO: Persist database
        database_url: Some("sqlite::memory:".into()),
        ..Default::default()
    };

    log::debug!("Start aquadoggo node with configuration {:?}", config);

    // Start p2panda node in async runtime
    let node = Node::start(config).await;

    tokio::select! {
        _ = node.on_exit() => (),
    }

    // Wait until all tasks are gracefully shut down and exit
    log::debug!("Shut down aquadoggo node");
    node.shutdown().await;
}

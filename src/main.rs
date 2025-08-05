mod bluetooth;

use axum::{routing::{get, post}, Router, Json};
use axum::response::Html;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum BluetoothState {
    On,
    Off,
}

#[derive(Serialize, Deserialize)]
struct BluetoothStatus {
    status: BluetoothState, 
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum RequestAction {
    Toggle,
}

#[derive(Deserialize)]
struct ControlRequest {
    action: RequestAction, 
}

async fn get_bluetooth_status() -> Json<BluetoothStatus> {
    // TODO: 实际调用系统API获取蓝牙状态
    // let status = get_system_bluetooth_status().await;
    let status = BluetoothState::On; // 假设当前状态为 "on"
    Json(BluetoothStatus { status })
}

async fn control_bluetooth(Json(payload): Json<ControlRequest>) -> Json<BluetoothStatus> {
    match payload.action {
        RequestAction::Toggle => {
            // TODO: 实际调用系统API切换蓝牙状态
            dbg!("Toggling Bluetooth status");
        },
        
    }
    
    // 获取更新后的状态
    // let status = get_system_bluetooth_status().await;
    let status = BluetoothState::Off; // TODO: 假设切换后状态为 "off"
    dbg!("Updated Bluetooth status: {}", &status);

    Json(BluetoothStatus { status })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/bluetooth/status", get(get_bluetooth_status))
        .route("/api/bluetooth/control", post(control_bluetooth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:500")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root_handler() -> Html<&'static str> {
    Html(include_str!("../assets/index.html"))
}

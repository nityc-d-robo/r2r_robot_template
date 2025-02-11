#[allow(unused_imports)]
use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::common_interfaces::sensor_msgs,
};
use safe_drive::msg::common_interfaces::geometry_msgs::msg;
use std::{cell::RefCell, rc::Rc};

/// 
pub struct WheelController {
    // 各ホイールの設定（未実装）
}

impl WheelController {
    /// 新しい `WheelController` インスタンスを作成する
    pub fn new() -> Self {
        // 未実装
        unimplemented!()
    }

    /// `cmd_vel` メッセージを受け取り、ホイールを制御する
    ///
    /// # 引数
    /// - `linear_x`: 前後方向の速度
    /// - `linear_y`: 左右方向の速度
    /// - `angular_z`: 旋回速度
    ///
    /// # 返り値
    /// - ホイールごとの制御値（未実装）
    ///
    /// # 注意
    /// - `cmd_vel` の値を適切に正規化し、最大速度を超えないようにする
    /// - 必要に応じて `UDP` や `PWM` 送信を実装する
    pub fn control_wheels(&self, linear_x: f64, linear_y: f64, angular_z: f64) {
        // 未実装
        unimplemented!()
    }
}

/// コントローラーの入力を処理する構造体
pub struct ControllerInput {
    // 内部データ（未実装）
}

impl ControllerInput {
    /// 新しい `ControllerInput` インスタンスを作成する
    pub fn new() -> Self {
        // 未実装
        unimplemented!()
    }

    /// `Joy` メッセージを受け取り、ボタンの状態を取得する
    ///
    /// # 引数
    /// - `joy_msg`: ROS 2 の `sensor_msgs::msg::Joy` メッセージ
    ///
    /// # 返り値
    /// - 各ボタンの押下状態（未実装）
    ///
    /// # 注意
    /// - ボタンごとの定義は `DualShock4Interface` などを参考にする
    /// - `pressed_*` メソッドを定義し、特定のボタンの押下状態を取得しやすくする
    pub fn process_joy_input(&mut self, joy_msg: &sensor_msgs::msg::Joy) {
        // 未実装
        unimplemented!()
    }
}

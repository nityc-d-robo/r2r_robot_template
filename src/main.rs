use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    pr_info,
};

use safe_drive::msg::common_interfaces::geometry_msgs::msg;

fn main() -> Result<(), DynError> {
    let ctx = Context::new()?;
    let node = ctx.create_node("r2r_robot_template", None, Default::default())?;
    
    // cmd_velノードを購読する
    let subscriber = node.create_subscriber::<msg::Twist>("cmd_vel", None)?;
    let mut selector = ctx.create_selector()?;

    selector.add_subscriber(subscriber, {
        Box::new(move |msg| {
            let _logger = Logger::new("r2r_robot_template");

            pr_info!(
                _logger,
                "{:?}",
                &[msg.linear.x, msg.linear.y, msg.angular.z]
            );
        })
    });

    loop {
        selector.wait()?;
    }
}

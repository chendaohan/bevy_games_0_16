use bevy::{
    app::ScheduleRunnerPlugin,
    ecs::{
        self,
        error::{ErrorContext, GLOBAL_ERROR_HANDLER},
        query::QuerySingleError,
    },
    prelude::*,
};

// 自定义统一 ECS 错误处理器
fn my_error_handler(error: BevyError, ctx: ErrorContext) {
    if ctx.name().ends_with("unifield_error_handling") {
        info!("{} 函数出错了", ctx.name());
        return;
    }

    // 在 error 日志级别记录错误
    ecs::error::error(error, ctx);
}

fn main() -> AppExit {
    // 设置统一 ECS 错误处理器
    // 需要 configurable_error_handler feature
    // 引擎提供的错误处理器：
    // panic ：错误将导致恐慌
    // error ：在 error 日志级别记录错误
    // warn ：在 warn 日志级别记录错误
    // info ：在 info 日志级别记录错误
    // debug ：在 debug 日志级别记录错误
    // trace ：在 trace 日志级别记录错误
    // ignore ：忽略错误
    GLOBAL_ERROR_HANDLER
        .set(my_error_handler)
        .expect("只能设置一次");

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_systems(Startup, setup)
        .add_systems(Update, unifield_error_handling)
        .add_systems(Update, separate_error_handling.pipe(separate_error_handler))
        .run()
}

#[derive(Component)]
struct MyPlayer;

// 命令统一 ECS 错误处理
fn setup(mut commands: Commands) {
    commands.queue(|world: &mut World| -> Result {
        world
            .spawn((Name::new("MyPlayer"), MyPlayer))
            .observe(observer_unifield_error_handling);

        Ok(())
    });
}

// 系统统一 ECS 错误处理
fn unifield_error_handling(players: Query<&Name, With<MyPlayer>>) -> Result {
    let name = players.single()?;
    info!("player name: {name}");

    Ok(())
}

// 观察者统一 ECS 错误处理
fn observer_unifield_error_handling(
    _trigger: Trigger<OnAdd>,
    players: Query<&Name, With<MyPlayer>>,
) -> Result {
    let name = players.single()?;
    info!("player name: {name}");

    Ok(())
}

//  单独错误处理
fn separate_error_handling(players: Query<&Name, With<MyPlayer>>) -> Result<(), QuerySingleError> {
    let name = players.single()?;
    info!("palyer name: {name}");

    Ok(())
}

// 单独错误处理器
fn separate_error_handler(In(result): In<Result<(), QuerySingleError>>) {
    if let Err(error) = result {
        info!("error: {error}");
    }
}

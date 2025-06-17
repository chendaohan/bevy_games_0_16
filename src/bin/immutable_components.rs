use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin,
    ecs::{component::HookContext, world::DeferredWorld},
    log::LogPlugin,
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(250))),
            LogPlugin::default(),
        ))
        .insert_resource(TotalSum(0))
        .add_systems(Startup, spawn_sum_me)
        .add_systems(Update, (modify_sum_me, print_total_sum))
        .run()
}

#[derive(Component)]
#[component(
    immutable,
    on_insert = add_when_inserting,
    on_remove = subtract_when_removing
)]
struct SumMe(u32);

// 在插入 SumMe 时将值加到 TotalSum 中
fn add_when_inserting(mut world: DeferredWorld, context: HookContext) {
    if let Some(sum_me) = world.get::<SumMe>(context.entity) {
        world.resource_mut::<TotalSum>().0 += sum_me.0;
    }
}

// 在移除 SumMe 时将值从 TotalSum 中减去
fn subtract_when_removing(mut world: DeferredWorld, context: HookContext) {
    if let Some(sum_me) = world.get::<SumMe>(context.entity) {
        world.resource_mut::<TotalSum>().0 -= sum_me.0;
    }
}

// 存储所有 SumMe 值的总和
#[derive(Resource)]
struct TotalSum(u32);

// 生成 SumMe
fn spawn_sum_me(mut commands: Commands) {
    commands.spawn(SumMe(5));
    commands.spawn(SumMe(3));
}

// 修改 SumMe
fn modify_sum_me(mut commands: Commands, query: Query<(Entity, &SumMe)>) {
    for (entity, sum_me) in query {
        commands.entity(entity).insert(SumMe(sum_me.0 + 1));
    }
}

// 打印 TotalSum
fn print_total_sum(total_sum: Res<TotalSum>) {
    info!("total_sum: {}", total_sum.0);
}

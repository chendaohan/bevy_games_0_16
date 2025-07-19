use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_once()),
            LogPlugin::default(),
        ))
        // 注册泛型组件
        .register_type::<GenericComponent<i32>>()
        .add_systems(Startup, setup)
        .add_systems(Update, (clone_enemies, clone_generic_components))
        .run()
}

#[derive(Component, Clone)]
#[require(Health(100.0), Attack(10.0))]
struct Enemy;

#[derive(Component, Clone)]
struct Health(f32);

#[derive(Component, Clone)]
struct Attack(f32);

#[derive(Component, Clone)]
struct Weapon;

fn setup(mut commands: Commands) {
    // 生成一个空实体，为这个实体添加观察者。
    // 在这个空实体中插入 Enemy，和一个有 Weapon 组件的子实体,Enemy 会触发观察者。
    commands
        .spawn_empty()
        .observe(|_trigger: Trigger<OnAdd, Enemy>| {
            info!("Add Enemy!");
        })
        .insert((Enemy, Children::spawn_one(Weapon)));

    // 生成泛型组件
    commands.spawn(GenericComponent::<i32> { value: 0 });
}

// 泛型组件必须实现反射，才能被克隆
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct GenericComponent<T> {
    value: T,
}

// 克隆敌人
fn clone_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for entity in enemies {
        //  将此实体中的可克隆组件，克隆到新生成的实体中
        commands.entity(entity).clone_and_spawn();

        let target = commands.spawn_empty().id();
        // 将指定组件克隆到目标实体
        commands
            .entity(entity)
            .clone_components::<(Enemy, Health, Attack)>(target);

        // 克隆到新实体并配置克隆行为
        commands.entity(entity).clone_and_spawn_with(|builder| {
            builder
                // 所有组件都不允许克隆
                .deny_all()
                // 允许克隆 Enemy
                .allow::<Enemy>()
                // 可以克隆子实体
                .linked_cloning(true); // 默认为 false
        });

        let target = commands.spawn_empty().id();
        // 克隆到目标实体并配置克隆行为
        commands.entity(entity).clone_with(target, |builder| {
            builder
                // 将克隆变成移动
                .move_components(true) // 默认为 false
                // 不允许克隆 Health
                .deny::<Health>()
                // 允许克隆所有可克隆组件
                .allow_all()
                // 如果被克隆的实体在观察者的观察列表中，那么就将新实体也添加到观察者的观察列表中
                .add_observers(true); // 默认为 false
        });

        let target = commands.spawn_empty().id();
        // 将指定组件移动到目标实体中
        commands
            .entity(entity)
            .move_components::<(Enemy, Health, Attack)>(target);
    }
}

// 克隆泛型组件
fn clone_generic_components(
    mut commands: Commands,
    generic_components: Query<Entity, With<GenericComponent<i32>>>,
) {
    for entity in generic_components {
        commands.entity(entity).clone_and_spawn();
    }
}

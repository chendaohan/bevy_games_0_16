use bevy::{
    app::ScheduleRunnerPlugin,
    ecs::{
        relationship::RelatedSpawner,
        spawn::{SpawnIter, SpawnWith},
    },
    log::LogPlugin,
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ScheduleRunnerPlugin::run_once()),
            LogPlugin::default(),
        ))
        .add_systems(Startup, (spawn_children, spawn_liked_by))
        .run()
}

#[derive(Component)]
#[relationship(relationship_target = LikedBy)]
struct Likes(Entity);

#[derive(Component)]
#[relationship_target(relationship = Likes)]
struct LikedBy(Vec<Entity>);

// 父子级关系生成 api
fn spawn_children(mut commands: Commands) {
    //  0   0-0     0-0-0
    //              0-0-1
    //      0-1
    commands.spawn((
        Name::new("0"),
        children![
            (
                Name::new("0-0"),
                children![Name::new("0-0-0"), Name::new("0-0-1"),]
            ),
            Name::new("0-1"),
        ],
    ));

    // 上面宏展开后的样子
    commands.spawn((
        Name::new("0"),
        Children::spawn((
            Spawn((
                Name::new("0-0"),
                Children::spawn((Spawn(Name::new("0-0-0")), Spawn(Name::new("0-0-1")))),
            )),
            Spawn(Name::new("0-1")),
        )),
    ));

    commands.spawn((
        Name::new("0"),
        Children::spawn((
            // 用闭包的方式生成子实体
            SpawnWith(|parent: &mut ChildSpawner| {
                parent.spawn((
                    Name::new("0-0"),
                    Children::spawn(
                        // 用迭代器的方式生成子实体
                        SpawnIter(["0-0-0", "0-0-1"].into_iter().map(Name::new)),
                    ),
                ));
                parent.spawn(Name::new("0-1"));
            }),
        )),
    ));
}

// 自定义关系生成 api
fn spawn_liked_by(mut commands: Commands) {
    commands.spawn((
        Name::new("0"),
        related!(LikedBy [
            (
                Name::new("0-0"),
                related!(LikedBy [
                    Name::new("0-0-0"),
                    Name::new("0-0-1"),
                ]),
            ),
            Name::new("0-1"),
        ]),
    ));

    commands.spawn((
        Name::new("0"),
        LikedBy::spawn((
            Spawn((
                Name::new("0-0"),
                LikedBy::spawn((Spawn(Name::new("0-0-0")), Spawn(Name::new("0-0-1")))),
            )),
            Spawn(Name::new("0-1")),
        )),
    ));

    commands.spawn((
        Name::new("0"),
        LikedBy::spawn(SpawnWith(|parent: &mut RelatedSpawner<Likes>| {
            parent.spawn((
                Name::new("0-0"),
                LikedBy::spawn(SpawnIter(["0-0-0", "0-0-1"].into_iter().map(Name::new))),
            ));
            parent.spawn(Name::new("0-1"));
        })),
    ));
}

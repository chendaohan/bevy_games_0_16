mod relation {
    use bevy::prelude::*;

    #[derive(Component)]
    #[relationship(relationship_target = LikedBy)]
    pub struct Likes(pub Entity); // 不可变组件，事实来源

    #[derive(Component)]
    #[relationship_target(relationship = Likes)]
    pub struct LikedBy(Vec<Entity>); // 可变组件，Vec 的访问权限必须为私有
}

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use relation::*;


fn main() -> AppExit {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_once()),
            LogPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (print_liked_by_of_e0, print_any_likes).chain())
        .run()
}

fn setup(mut commands: Commands) {
    // (Name, LikedBy)
    let e0 = commands.spawn(Name::new("0")).id();
    // (Likes, LikedBy, Name)
    let e1 = commands.spawn((Likes(e0), Name::new("0-0"))).id();
    // (Likes, Name)
    commands.spawn((Likes(e0), Name::new("0-1")));
    // (Likes, Name)
    commands.spawn((Likes(e1), Name::new("0-0-0")));

    //  0   0-0     0-0-0
    //      0-1
}

fn print_liked_by_of_e0(liked_by: Single<&LikedBy, Without<Likes>>, names: Query<&Name>) -> Result {
    for entity in liked_by.iter() {
        let name = names.get(entity)?;
        info!("liked by name: {name}");
    }

    Ok(())
}

fn print_any_likes(likes: Query<&Likes>, names: Query<&Name>) -> Result {
    let entity = likes.into_iter().next().ok_or("None")?.0;
    let name = names.get(entity)?;
    info!("likes name: {name}");
    Ok(())
}

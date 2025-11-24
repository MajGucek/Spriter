
    use bevy::{
        app::AppExit,
        prelude::*,
    };
    use crate::button_system::button_system;
    use crate::GameState;

    #[derive(Component)]
    struct SelectedOption;

    pub fn display_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Display), display_setup)
            .add_systems(FixedUpdate, (button_system::<SelectedOption>, display_action).run_if(in_state(GameState::Display)));

    }

    fn display_setup(mut commands: Commands) {
        let button_node = Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        commands.spawn((
            DespawnOnExit(GameState::Display),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("Telemetry here!"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                    ),
                    (

                    ),
                    (

                    ),
                ]
        )]));

    }

    fn display_action() {

    }
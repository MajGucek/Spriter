use crate::button_system::button_system;
use crate::GameState;
use bevy::{
    app::AppExit,
    prelude::*,
};
use bevy_simple_text_input::{TextInput, TextInputSubmitMessage, TextInputSystem};

#[derive(Component)]
    struct SelectedOption;

    #[derive(Component)]
    enum MenuButtonAction {
        Proceed,
        Quit,
    }

    pub fn menu_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(FixedUpdate, (button_system::<SelectedOption>, menu_action).run_if(in_state(GameState::Menu)))
            .add_systems(Update, listener.after(TextInputSystem))
        ;
    }

    fn menu_setup(mut commands: Commands) {
        let button_node = Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        commands.spawn((
            DespawnOnExit(GameState::Menu),
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
                        Text::new("RBR-Telemetry-Viewer"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(Color::WHITE),
                        MenuButtonAction::Proceed,
                        children![(
                            Text::new("Next")
                        )]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(Color::WHITE),
                        MenuButtonAction::Quit,
                        children![(
                            Text::new("Quit")
                        )]
                    ),
                    (
                        TextInput,
                        Node {
                            padding: UiRect::all(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::BLACK)
                    ),
                ]
        )]));

    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                    MenuButtonAction::Proceed => {
                        game_state.set(GameState::Display);
                    }
                }
            }
        }
    }

 fn listener(mut events: MessageReader<TextInputSubmitMessage>) {
     for event in events.read() {
         info!("{:?} submitted: {}", event.entity, event.value);
     }
 }

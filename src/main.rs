use bevy::{
    ecs::system::{SystemParam, SystemParamItem},
    prelude::*,
};

const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_main_menu)
        .add_system(handle_buttons(handle_button_click))
        .run();
}

#[derive(Clone, Copy, Component, Debug)]
enum Buttons {
    StartGame,
}

fn handle_buttons<T, H, HOut, HParam, HMarker>(
    mut on_click: H,
) -> impl FnMut(
    Query<(&Interaction, &mut BackgroundColor, &T), (Changed<Interaction>, With<Button>)>,
    ParamSet<(SystemParamItem<HParam>,)>,
)
where
    T: Component + Clone,
    H: SystemParamFunction<T, HOut, HParam, HMarker>,
    HParam: SystemParam,
{
    move |mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &T),
        (Changed<Interaction>, With<Button>),
    >,
          mut params: ParamSet<(SystemParamItem<HParam>,)>| {
        for (interaction, mut color, item) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON_COLOR.into();
                    on_click.run(item.clone(), params.p0()); // TODO: Remove .clone() somehow
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }
}

fn handle_button_click(In(button): In<Buttons>, mut windows: ResMut<Windows>) {
    match button {
        Buttons::StartGame => {
            info!("Button clicked");
            windows.primary_mut().close();
        }
    }
}

fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle::default());

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands
        // fullscreen vertical layout
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                align_content: bevy::ui::AlignContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Center,
                        align_content: bevy::ui::AlignContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        size: Size::new(Val::Auto, Val::Auto),
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                                    padding: UiRect::horizontal(Val::Px(12.)),
                                    margin: UiRect::vertical(Val::Px(4.0)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            Buttons::StartGame,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Start Game", text_style.clone()));
                        });
                });
        });
}

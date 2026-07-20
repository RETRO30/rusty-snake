use bevy::prelude::*;

const KEY_BORDER: Color = Color::WHITE;
const KEY_TEXT: Color = Color::WHITE;
const LABEL_TEXT: Color = Color::srgb(0.75, 0.75, 0.8);

fn spawn_key_cap(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, label: &str) {
    parent
        .spawn((
            Node {
                width: Val::Px(36.0),
                height: Val::Px(36.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(KEY_BORDER),
        ))
        .with_children(|key| {
            key.spawn((
                Text::new(label),
                TextFont {
                    font: FontSource::Handle(font.clone()),
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(KEY_TEXT),
            ));
        });
}

fn spawn_hint_group(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    keys: &[&str],
    label: &str,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|group| {
            group
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(6.0),
                    ..default()
                })
                .with_children(|row| {
                    for key in keys {
                        spawn_key_cap(row, font, key);
                    }
                });
            group.spawn((
                Text::new(label),
                TextFont {
                    font: FontSource::Handle(font.clone()),
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(LABEL_TEXT),
            ));
        });
}

pub fn spawn_hints(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(32.0),
            align_items: AlignItems::Start,
            ..default()
        })
        .with_children(|row| {
            spawn_hint_group(row, font, &["↑", "↓", "←", "→"], "Move");
            spawn_hint_group(row, font, &["Esc"], "Pause");
        });
}

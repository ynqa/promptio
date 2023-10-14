use std::iter::FromIterator;

use anyhow::Result;

use promkit::{
    crossterm::style::Color,
    editor::{ItemPickerBuilder, Mode, ReadlineBuilder, TextBuilder},
    item_box::ItemBox,
    style::ContentStyleBuilder,
    suggest::Suggest,
    Prompt,
};

fn main() -> Result<()> {
    let mut p = Prompt::new(vec![
        TextBuilder::new("Type Here")
            .style(
                ContentStyleBuilder::new()
                    .foreground_color(Color::Green)
                    .build(),
            )
            .build()?,
        ReadlineBuilder::default()
            .style(
                ContentStyleBuilder::new()
                    .foreground_color(Color::DarkYellow)
                    .build(),
            )
            .cursor_style(
                ContentStyleBuilder::new()
                    .background_color(Color::DarkBlue)
                    .build(),
            )
            .label_style(
                ContentStyleBuilder::new()
                    .foreground_color(Color::DarkGreen)
                    .build(),
            )
            .suggest(Suggest::from_iter(["promkit", "ynqa"]))
            .build()?,
        ReadlineBuilder::default().mode(Mode::Overwrite).build()?,
        ReadlineBuilder::default()
            .mask('*')
            .cursor_style(
                ContentStyleBuilder::new()
                    .background_color(Color::Blue)
                    .build(),
            )
            .build()?,
        ItemPickerBuilder::new(ItemBox::from_iter(0..100))
            .cursor_style(
                ContentStyleBuilder::new()
                    .foreground_color(Color::Magenta)
                    .build(),
            )
            .lines(5)
            .build()?,
        ItemPickerBuilder::new(ItemBox::from_iter(0..100))
            .cursor_style(
                ContentStyleBuilder::new()
                    .foreground_color(Color::Magenta)
                    .build(),
            )
            .build()?,
    ]);
    loop {
        let line = p.run()?;
        println!("result: {:?}", line);
    }
}

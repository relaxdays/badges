use badges::{BadgeBuilder, BadgeColor, BadgeStyle};
use clap::{builder::TypedValueParser, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Badge label (left side)
    label: String,
    /// Badge message (right side)
    message: String,
    /// Background color for the label
    #[arg(short, long, default_value = "grey", value_parser = color_parser())]
    label_color: BadgeColor,
    /// Background color for the message
    #[arg(short, long, default_value = "light-grey", value_parser = color_parser())]
    message_color: BadgeColor,
    /// Badge style
    #[arg(short, long, default_value = "flat", value_enum)]
    style: BadgeStyle,
}

fn main() {
    let cli = Cli::parse();
    let result = BadgeBuilder::new()
        .style(cli.style)
        .label(&cli.label)
        .label_color(cli.label_color)
        .message(&cli.message)
        .message_color(cli.message_color)
        .render();
    match result {
        Ok(badge) => {
            println!("{}", badge);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn color_parser() -> impl clap::builder::TypedValueParser {
    clap::builder::StringValueParser::new().try_map(|v| BadgeColor::try_from(v.as_str()))
}

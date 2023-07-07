use yansi::{Paint, Style, Color::*};

static ALERT: Style = Yellow.on_white().bold().underline();

fn main() {
    dbg!(std::mem::size_of::<yansi::Color>());
    dbg!(std::mem::size_of::<Option<yansi::Color>>());
    dbg!(std::mem::size_of::<yansi::Condition>());
    dbg!(std::mem::size_of::<Option<yansi::Condition>>());
    dbg!(std::mem::size_of::<yansi::Style>());
    dbg!(std::mem::size_of::<yansi::Attribute>());
    // dbg!(std::mem::size_of::<yansi::Set<yansi::Attribute>>());

    // let style = (Blue | Bold | Blink | Italic).bg(White);
    // let style = Blue.bold().blink().italic().on_white();
    let x = "this is x".red().italic();
    let y = "now y".blue().bold().blink().italic().bg(White);
    // let z = "foo".apply(Blue.bold()).bg(Red);
    let z = "finally z".blue().bold().bg(Red);
    println!("{:?}, {:?}, {:?}", x, y, z);
    println!("xyz {}, {}, {}", x, y, z);
    println!("bright xyz {}, {}, {}", x.bright(), y.bright(), z.bright().dim());

    println!("{}, {}, {}", "blue".blue(), "dim blue".blue().dim(), "bright blue".bright().blue());
    println!("{}, {}, {}", "red".red(), "dim red".red().dim(), "bright red".bright().red());
    println!("{}, {}, {}", "yellow".yellow(), "dim yellow".yellow().dim(), "bright yellow".bright().yellow());
    println!("{}, {}, {}", "green".green(), "dim green".green().dim(), "bright green".bright().green());
    println!("{}, {}, {}", "magenta".magenta(), "dim magenta".magenta().dim(), "bright magenta".bright().magenta());
    println!("{}, {}, {}", "cyan".cyan(), "dim cyan".cyan().dim(), "bright cyan".bright().cyan());
    println!("{}, {}, {}", "black".black(), "dim black".black().dim(), "bright black".bright().black());
    // println!("{}", std::mem::size_of::<Color>());
    // println!("{}", std::mem::size_of::<yansi::Style>());

    let stop = Red;
    let wait = Yellow.bold().underline();
    let go = Green.italic().on_black();
    println!("Testing, {}, {}, {}!",
        1.paint(ALERT).paint(stop), 2.paint(wait), "3".paint(go).mask());

    println!("Testing, {}, {}, {}!",
        1.red(),
        2.yellow().bold().underline(),
        "3".green().on_white().italic());

    let normal = "Normal".primary().on_black();
    println!("{}", normal);
    println!("{}", normal.on_bright());
    println!("{}", normal.invert().on_bright());
    println!("{}", normal.invert().invert().on_bright());
    println!("{}", normal.strike().blink().rapid_blink().conceal());
    println!("{}", "primary on primary".primary().on_primary().invert());

    // // use yansi::hyperlink::HyperlinkExt;
    // println!("go to {}, please", 8.green().link("https://google.com").italic());

    let inner = format!("{} and {}", "Stop".red(), "Go".green());
    println!("Hey! {}", inner.blue());

    println!("Testing, {}, {}, {}!",
        "Ready".bold(),
        "Set".yellow().italic().bold(),
        "STOP".white().on_red().bright().underline().bold());
}

use std::env;

use elmio_core::page::Page;
use elmio_tailwind_core::home_page;
use url::Url;

fn main() {
    let env_args: Vec<String> = env::args().collect();

    let args: Vec<&str> = env_args.iter().map(|s| s.as_ref()).collect();

    match args[1..] {
        ["home_page"] => {
            let page = home_page::HomePage {
                current_url: Url::parse("http://localhost/").unwrap(),
            };
            print_html(page);
        }

        _ => {
            println!("Invalid command");
        }
    }
}

fn print_html<Model, Msg, Markup>(page: impl Page<Model, Msg, Markup>) {
    let (model, _effects) = page.init().expect("Failed to init page");
    let markup = page.view(&model);
    println!("{}", page.render_page(markup));
}

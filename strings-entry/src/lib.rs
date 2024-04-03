use proc_macro::*;

#[proc_macro_attribute]
pub fn create_game(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fun = Ident::new("create_game", Span::call_site());
    for i in item.clone() {
        match i {
            TokenTree::Ident(i) => {
                if i.to_string() == "fn" {
                    continue;
                }
                fun = i;
                break;
            },
            _ => {}
        }
    }
    format!("
    // Externally-defined function to create a game.
    {item}

    // The main entry point of the application.
    fn main() -> ::std::result::Result<(),String> {{
        // Request the game instance from the application.
        let game: Box<dyn ::strings::game::Game> = match {fun}() {{
            Ok(g) => g,
            Err(e) => {{
                ::strings::fatal!(\"Could not create game!\");
                return Err(e);
            }}
        }};

        // Initialization.
        match ::strings::core::app::App::create(game, ::strings::core::app::AppConfig {{
            x: 100,
            y: 100,
            width: 1280,
            height: 720,
            name: \"Strings Engine Testbed\".to_string(),
        }}) {{
            Ok(app) => if let Err(e) = app.run() {{
                ::strings::info!(\"Application failed to create!.\");
                return Err(e);
            }}
            Err(e) => {{
                ::strings::info!(\"Application did not shutdown gracefully.\");
                return Err(e);
            }}
        }};
        Ok(())
    }}").parse().unwrap()
}
use proc_macro::*;

#[proc_macro_attribute]
pub fn create_game(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut iter = item.clone().into_iter();
    iter.next();
    let fun = iter.next().unwrap();
    format!(
        "
        // Externally-defined function to create a game.
        {item}

        // The main entry point of the application.
        fn main() -> ::std::result::Result<(),String> {{
            // Request the game instance from the application.
            let (game, app_config): (Box<dyn ::strings::game::Game>, ::strings::core::app::AppConfig) = match {fun}() {{
                Ok(g) => g,
                Err(e) => {{
                    ::strings::fatal!(\"Could not create game!\");
                    return Err(e);
                }}
            }};

            // Initialization.
            match ::strings::core::app::App::create(game, app_config) {{
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
        }}"
    )
    .parse()
    .unwrap()
}

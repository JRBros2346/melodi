use proc_macro::*;

#[proc_macro]
pub fn create_game(item: TokenStream) -> TokenStream {
    format!(
        "// Externally-defined function to create a game.
        fn create() -> ::std::result::Result<(::std::boxed::Box<dyn ::strings::game::Game>, ::strings::core::app::AppConfig),::strings::game::GameError> {{
            {item}
        }}

        // The main entry point of the application.
        fn main() -> ::std::result::Result<(),::std::boxed::Box<dyn ::std::error::Error>> {{
            // Request the game instance from the application.
            let (game, app_config): (::std::boxed::Box<dyn ::strings::game::Game>, ::strings::core::app::AppConfig) = match create() {{
                Ok(g) => g,
                Err(e) => {{
                    ::strings::fatal!(\"Could not create game!\");
                    return Err(::std::boxed::Box::new(e));
                }}
            }};

            // Initialization.
            match ::strings::core::app::App::create(game, app_config) {{
                Ok(app) => if let Err(e) = app.run() {{
                    ::strings::info!(\"Application failed to create!.\");
                    return Err(::std::boxed::Box::new(e));
                }}
                Err(e) => {{
                    ::strings::info!(\"Application did not shutdown gracefully.\");
                    return Err(::std::boxed::Box::new(e));
                }}
            }};
            Ok(())
        }}"
    )
    .parse()
    .unwrap()
}

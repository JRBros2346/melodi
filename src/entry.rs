#[macro_export]
macro_rules! create_game {
    ($body:block) => {
        // Externally-defined function to create a game.
        fn create() -> ::std::result::Result<
            (
                ::std::boxed::Box<dyn $crate::game::Game>,
                $crate::core::app::AppConfig,
            ),
            $crate::game::GameError,
        >
        $body


        // The main entry point of the application.
        fn main() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {
            $crate::core::mem::init();

            // Request the game instance from the application.
            let (game, app_config): (
                ::std::boxed::Box<dyn $crate::game::Game>,
                $crate::core::app::AppConfig,
            ) = match create() {
                Ok(g) => g,
                Err(e) => {
                    $crate::fatal!("Could not create game!");
                    return Err(::std::boxed::Box::new(e));
                }
            };

            // Initialization.
            match $crate::core::app::App::create(game, app_config) {
                Ok(app) => {
                    if let Err(e) = app.run() {
                        $crate::info!("Application failed to create!.");
                        return Err(::std::boxed::Box::new(e));
                    }
                }
                Err(e) => {
                    $crate::info!("Application did not shutdown gracefully.");
                    return Err(::std::boxed::Box::new(e));
                }
            };

            $crate::core::mem::close();
            Ok(())
        }
    };
}

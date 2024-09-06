use zed_extension_api as zed;

struct MyExtension {
    // ... state
}

impl zed::Extension for MyExtension {
    fn new() -> Self {
        MyExtension {
            // ... initialize state
        }
    }
}

zed::register_extension!(MyExtension);

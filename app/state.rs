use minijinja::Environment;

pub struct AppState {
    pub env: Environment<'static>,
}

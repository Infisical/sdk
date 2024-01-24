extern crate log;
use infisical_json::client::Client as JsonClient;
use pyo3::prelude::*;

#[pyclass]
pub struct InfisicalClient(JsonClient);

#[pymethods]
impl InfisicalClient {
    #[new]
    pub fn new(settings_string: Option<String>, debug: Option<bool>) -> Self {
        if debug.unwrap_or(false) == true {
            // This will only fail if another logger was already initialized, so we can ignore the result
            let _ = env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Debug)
                .try_init();
        }

        Self(JsonClient::new(settings_string))
    }

    #[pyo3(text_signature = "($self, command_input)")]
    fn run_command(&mut self, command_input: String) -> String {
        run_command(&mut self.0, &command_input)
    }
}

#[tokio::main]
async fn run_command(client: &mut JsonClient, input_str: &str) -> String {
    client.run_command(input_str).await
}

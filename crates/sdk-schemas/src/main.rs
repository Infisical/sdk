use std::{fs::File, io::Write};

use anyhow::Result;
use itertools::Itertools;
use schemars::{schema::RootSchema, schema_for};

macro_rules! write_schema_for {
    ($type:ty) => {
        let schema = schema_for!($type);

        let type_name = stringify!($type);
        let path: Vec<&str> = type_name.split("::").collect();
        let dir_path =
            String::from("support/schemas/") + &path.iter().take(path.len() - 2).join("/");

        write_schema(schema, dir_path, type_name.to_string())?;
    };
    ($path:literal, $type:ty) => {
        let schema = schema_for!($type);

        write_schema(
            schema,
            String::from("support/schemas/") + $path,
            stringify!($type).to_string(),
        )?;
    };
}

macro_rules! write_schema_for_response {
    ( $($type:ty),+ $(,)? ) => {
        $( write_schema_for!("response", infisical_json::response::Response<$type>); )+
    };
}

fn write_schema(schema: RootSchema, dir_path: String, type_name: String) -> Result<()> {
    let file_name = type_name
        .split("::")
        .last()
        .unwrap()
        .to_string()
        .trim_end_matches('>')
        .to_string()
        + ".json";

    let content = serde_json::to_string_pretty(&schema)?;
    let _ = std::fs::create_dir_all(&dir_path);
    let mut file = File::create(format!("{}/{}", dir_path, file_name))?;
    writeln!(&mut file, "{}", &content)?;
    Ok(())
}

fn main() -> Result<()> {
    // Input types for new Client
    write_schema_for!(infisical_json::client::ClientSettings);
    // Input types for Client::run_command
    write_schema_for!(infisical_json::command::Command);

    // Output types for Client::run_command
    // Only add structs which are direct results of SDK commands.
    write_schema_for_response! {
        infisical::manager::secrets::GetSecretResponse,
        infisical::manager::secrets::ListSecretsResponse,
        infisical::manager::secrets::UpdateSecretResponse,
        infisical::manager::secrets::DeleteSecretResponse,
        infisical::manager::secrets::CreateSecretResponse,
        infisical::auth::AccessTokenSuccessResponse,

        infisical::manager::cryptography::EncryptSymmetricResponse,
        infisical::manager::cryptography::DecryptSymmetricResponse,
        infisical::manager::cryptography::CreateSymmetricKeyResponse,
    };

    Ok(())
}

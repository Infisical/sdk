// use aws_config::meta::region::RegionProviderChain;
// use aws_config::{BehaviorVersion, SdkConfig};
// use aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput;
// use aws_sdk_sts::Client as AwsStsClient;

// use crate::error::{Error, Result};
// use crate::Client;

// use super::AccessTokenSuccessResponse;

// async fn assume_role(
//     config: &SdkConfig,
//     role_name: String,
//     session_name: Option<String>,
// ) -> Result<GetCallerIdentityOutput> {
//     let provider = aws_config::sts::AssumeRoleProvider::builder(role_name)
//         .session_name(session_name.unwrap_or("session1".into()))
//         .configure(config)
//         .build()
//         .await;

//     let local_config = aws_config::defaults(BehaviorVersion::v2024_03_28())
//         .credentials_provider(provider)
//         .load()
//         .await;
//     let client = AwsStsClient::new(&local_config);

//     let req = client.get_caller_identity();
//     let resp = req.send().await;

//     if let Err(e) = resp {
//         println!("Error: {:?}", e);
//         return Err(Error::UnknownErrorWithMessage {
//             message: e.to_string(),
//         });
//     }

//     let e = resp.as_ref().unwrap();

//     println!("UserID :               {}", e.user_id().unwrap_or_default());
//     println!("Account:               {}", e.account().unwrap_or_default());
//     println!("Arn    :               {}", e.arn().unwrap_or_default());
//     return Ok(e.clone());
// }

// pub async fn aws_iam_login(client: &mut Client) -> Result<()> {
//     let role_name = "i dont fucking know :)".to_string();
//     let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

//     let region = region_provider.region().await;

//     let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
//         .region(Some(region).unwrap())
//         .load()
//         .await;

//     let identity = assume_role(&config, role_name, None).await?;

//     Ok(())
// }

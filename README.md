# Acrab

This is a simple project about pubsub in rust using google cloud pubsub.

`let config = ClientConfig::default().with_auth().await.unwrap();` is using the with_auth() method to add authentication to an instance of ClientConfig.

In this case, the ClientConfig instance is being constructed using its static default() method, which returns an instance with default values, and then the with_auth() method is called to add the necessary authentication.

The with_auth() method uses the default Google Cloud Platform credentials obtained from the runtime environment, such as the GOOGLE_APPLICATION_CREDENTIALS and GOOGLE_CLOUD_PROJECT environment variables.

If you want to use a different authentication mechanism, you can use the with_credentials() method instead of with_auth().

`let client = Client::new(config);` is using the ClientConfig instance to create a Client instance.

## Environment Variables

- GOOGLE_APPLICATION_CREDENTIALS - The path to the credentials file to use for this invocation. If not set, the default credentials are used.
- GOOGLE_CLOUD_PROJECT - The project ID of the project to use for this invocation. If not set, the project ID from the credentials file is used.

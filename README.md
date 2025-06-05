# doppler-rs

[![Crates.io](https://img.shields.io/crates/v/doppler-rs.svg)](https://crates.io/crates/doppler-rs)
[![Documentation](https://docs.rs/doppler-rs/badge.svg)](https://docs.rs/doppler-rs)
[![License](https://img.shields.io/crates/l/doppler-rs.svg)](https://github.com/yourusername/doppler-rs)

An unofficial Rust client library for the [Doppler](https://doppler.com) API. Doppler is a secrets management platform that helps developers securely store, manage, and deploy environment variables and secrets across applications and environments.

## Features

- **Complete API Coverage**: Auto-generated from Doppler's OpenAPI specification
- **Async/Await Support**: Built on `reqwest` with full async support
- **Type Safety**: Strongly typed request/response models with `serde` serialization
- **Configurable**: Support for different authentication methods and custom configurations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
doppler-rs = "0.0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use doppler_rs::apis::configuration::Configuration;
use doppler_rs::apis::default_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the API client
    let config = Configuration {
        bearer_access_token: Some("your-doppler-token".to_string()),
        ..Default::default()
    };

    // List all projects
    let projects = default_api::projects_list(&config, None, None).await?;
    println!("Projects: {:?}", projects);

    Ok(())
}
```

## Authentication

The Doppler API supports several authentication methods:

### Service Tokens
```rust
let config = Configuration {
    base_path: "https://api.doppler.com".to_string(),
    bearer_access_token: Some("dp.st.your-service-token".to_string()),
    ..Default::default()
};
```

### Personal Tokens
```rust
let config = Configuration {
    base_path: "https://api.doppler.com".to_string(),
    bearer_access_token: Some("dp.pt.your-personal-token".to_string()),
    ..Default::default()
};
```

## API Examples

### Working with Projects

```rust
use doppler_rs::apis::default_api;
use doppler_rs::models::*;

// List all projects
let projects = default_api::projects_list(&config, None, None).await?;

// Create a new project
let project_request = ProjectsCreateRequest {
    name: "My New Project".to_string(),
    description: Some("A sample project".to_string()),
};
let project = default_api::projects_create(&config, project_request).await?;

// Get a specific project
let project = default_api::projects_get(&config, "project-name").await?;
```

### Working with Environments

```rust
// List environments for a project
let environments = default_api::environments_list(&config, "project-name").await?;

// Create a new environment
let env_request = EnvironmentsCreateRequest {
    name: "Development".to_string(),
    slug: "dev".to_string(),
    personal_configs: Some(false),
};
let environment = default_api::environments_create(&config, "project-name", env_request).await?;
```

### Working with Configs

```rust
// List configs for a project and environment
let configs = default_api::configs_list(&config, "project-name", Some("environment-name")).await?;

// Get secrets from a config
let secrets = default_api::configs_secrets_download(&config, "project-name", "config-name", None).await?;
```

## API Coverage

This client provides access to all Doppler API endpoints, including:

- **Projects**: Create, list, retrieve, update, and delete projects
- **Environments**: Manage project environments
- **Configs**: Handle configuration management
- **Secrets**: Retrieve and manage secrets
- **Service Tokens**: Manage API authentication tokens
- **Integrations**: Configure external service integrations
- **Audit Logs**: Access audit trail information
- **Users & Groups**: Manage team access and permissions

## Error Handling

The client uses standard Rust error handling patterns:

```rust
match default_api::projects_list(&config, None, None).await {
    Ok(projects) => println!("Found {} projects", projects.projects.len()),
    Err(e) => eprintln!("API error: {}", e),
}
```

## Configuration Options

The `Configuration` struct supports various options:

```rust
use doppler_rs::apis::configuration::Configuration;

let config = Configuration {
    base_path: "https://api.doppler.com".to_string(),
    user_agent: Some("MyApp/1.0".to_string()),
    bearer_access_token: Some("your-token".to_string()),
    ..Default::default()
};
```

## Development

This crate is auto-generated from Doppler's OpenAPI specification using the OpenAPI Generator. To regenerate the client:

1. Download the latest OpenAPI spec from Doppler
2. Run the code generation script: `./code-gen`

## Contributing

Contributions are welcome! Please note that this crate is auto-generated, so most changes should be made to the generation process rather than the generated code directly.

## License

This project is licensed under the Unlicense - see the generated code for details.

## Links

- [Doppler Documentation](https://docs.doppler.com/)
- [Doppler API Reference](https://docs.doppler.com/reference)
- [Crates.io](https://crates.io/crates/doppler-rs)
- [Documentation](https://docs.rs/doppler-rs)

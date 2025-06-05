# Integration Tests for Doppler Secrets API

This directory contains integration tests for the Doppler Secrets API endpoints. These tests make real API calls to the Doppler service.

## Prerequisites

Before running the integration tests, you need:

1. A valid Doppler API token
2. Access to a Doppler project and configuration

## Environment Variables

Set the following environment variables before running the tests:

### Required
- `DOPPLER_TOKEN`: Your Doppler API token (Bearer token)

### Optional
- `DOPPLER_TEST_PROJECT`: The project to use for testing (defaults to "test-project")
- `DOPPLER_TEST_CONFIG`: The configuration to use for testing (defaults to "dev")

## Running the Tests

### Run all integration tests
```bash
DOPPLER_TOKEN=your_api_token_here cargo test --test secrets_integration_tests
```

### Run specific tests
```bash
DOPPLER_TOKEN=your_api_token_here cargo test --test secrets_integration_tests test_secrets_list_integration
```

### Run with custom project/config
```bash
DOPPLER_TOKEN=your_api_token_here \
DOPPLER_TEST_PROJECT=my-project \
DOPPLER_TEST_CONFIG=production \
cargo test --test secrets_integration_tests
```

### Run without credentials (will skip tests that require authentication)
```bash
cargo test --test secrets_integration_tests
```

## Test Coverage

The integration tests cover the following secrets endpoints:

### Success Cases
- `secrets_list` - List all secrets in a configuration
- `secrets_list` with dynamic secrets - Include dynamic secrets with TTL
- `secrets_names` - Get secret names only
- `secrets_download` - Download secrets in ENV format
- `secrets_download` - Download secrets in JSON format with transformations
- `secrets_update_note` - Update a note on a secret

### Error Cases
- `secrets_get` - Get a non-existent secret (expects 404)
- `secrets_delete` - Delete a non-existent secret
- `secrets_update` - Update with invalid data (expects client error)
- Invalid authentication token (expects 401)
- Network error handling

### Configuration Tests
- Test default configuration values
- Test configuration creation

## Test Behavior

- Tests will gracefully skip if `DOPPLER_TOKEN` is not set
- Tests handle both success and expected error responses
- Tests use real API calls, so they may be affected by:
  - Network connectivity
  - API rate limits
  - Actual project/configuration existence
  - Token permissions

## Safety Considerations

These integration tests:
- Only read from the API (list, get, download operations)
- Test deletion of non-existent secrets
- Test updates with invalid data to trigger validation errors
- Do not create or modify real secrets in your projects

## Troubleshooting

### Tests are skipped
- Ensure `DOPPLER_TOKEN` environment variable is set
- Check that your token has the necessary permissions

### 404 Errors
- Verify that `DOPPLER_TEST_PROJECT` and `DOPPLER_TEST_CONFIG` exist
- Check that your token has access to the specified project/config

### 401 Errors
- Verify your token is valid and not expired
- Ensure the token has the necessary scopes for the operations being tested

### Rate Limiting
- Doppler API has rate limits; if tests fail due to rate limiting, wait and retry
- Consider running tests less frequently in CI/CD pipelines 
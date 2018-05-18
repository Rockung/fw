## Integration Testing

* Unit tests
  - test one module in isolation at a time
  - small for private code
* integraton tests
  - external to your crate
  - test public interfaces work correctly together

### code sharing between integration test

* separate crate per file in tests directory
* moduling for common code

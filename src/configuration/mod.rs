/// This modules is responsible for managing the configuration 
/// settings of the application. It centralizes the setup of various
/// application parameters and environtment-specific settings
/// -

/// - This module contains configuration settings related to the database
/// - Such as connection strings, pool sizes, and timeout settings. 
pub mod database;

/// - This module manages loggins configurations
/// - Including log levels, file paths, and format settings
pub mod logging;

// - This module responsible for mananging environtment entities
pub mod environtment;
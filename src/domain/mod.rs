// This module encalsulates the core business logic of the application, 
// focusing on entities and services that represent the problem domain
// It is independent of the infrastructure and interfaces,
// ensuring that business rules are the central concern
 


// - Contains the domain entities that represent the core data and concepts of the application
// - These entities are typically structs that define 
// - the essential data attribites and methods related to the domain
pub mod entities;

// - This module includes business logic that operates on domain entities.
// - It contain the rules and workflows that dictate how data is processed and managed within the domain
pub mod services;
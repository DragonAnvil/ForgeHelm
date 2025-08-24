// Utility Functions & Helpers

// Utils Module for storing helper functions and common code
// Best for code that doesn't fit into the other module categories
// Usually includes Error Handling, Validation Helpers, Formatting Functions
// Not much should reside here, if this gets too large, consider adding more specific modules

// 
pub mod errors;

// Re-Export commonly used types
pub use errors::AppError;
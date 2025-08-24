// HTTP Request Handlers (Controllers)

// Handlers are the bridge between HTTP Requests and Business Logic
// Responsible for:
// 1. Extracting Data from HTTP Requests (Path Parameters, Query Parameters, Body)
// 2. Calling the appropriate Service Method
// 3. Converting the Results into HTTP Responses

// Handlers should be thin, with minimal logic
// Delegate real work to Service Layer functions

pub mod user_handlers;
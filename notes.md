1. Setup Rust project, added dependencies

2. init.sql updated for full table schemas
    rest.bat added for database reseting and setup

3. config.rs setup
4. db.rs setup
5. models.rs setup
6. queries.rs setup
7. handlers.rs setup
8. routes.rs setup
9. main.rs setup
10. init.sql modified
11. crud.bat created to test API endpoints in terminal
12. frontend added, dev.bat added to script starting backend & frontend

13. models.rs updated for workspaces, workspace_tools, and spreadsheet Structs

14. queries.rs updated for workspace queries

15. handlers.rs updated for workspace handlers

16. routes.rs updated for workspace routes

17. actix_cors added to main.rs to allow frontend comms with backend

Flowchart
Routes.rs > Handlers.rs > Query.rs > Models.rs

CRUD to Setup
*   Workspaces
    GET
    POST
    PUT
    DELETE
*   Workspace Tools
    GET
    POST
    PUT
    DELETE
*   Spreadsheets
    GET
    POST
    PUT
    DELETE

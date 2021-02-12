# TodoAPI (WIP)

## What is this?

This is a fully featured todo-list (REST) API built using the actix-web framework and the SQLx database toolkit, it has refresh token authentication and separates the todos into categories (essentially lists). For now, this is entirely a personal project to get me started with backend in rust and I don't intend on deploying it.

## Features:

- [ ] Refresh Token authentication:
  - [ ] Registration - creating a user (password hashing with argon2)
  - [ ] Login - checking login details and then generating refresh token (stored in redis) + access token (json web token) and sending them back in cookies.
  - [ ] Refresh - taking in the refresh token and checking its existence + expiry in the redis store and if valid, sending back an access token.
- [ ] Basic Category-related routes (all require access token auth):
  - [ ] Create category
  - [ ] Public categories (i.e. multiple users collaborating)\*
  - [ ] Get all categories
  - [ ] Get single category (with child todos joined)
  - [ ] Update category details
  - [ ] Delete category
- [ ] Basic Todo-related routes:
  - [ ] Create todo
  - [ ] Get all todos
  - [ ] Get all todos under category
  - [ ] Toggle todo completed
  - [ ] Update todo details
  - [ ] Delete todo
- [ ] Search (full text) + filter routes:
  - [ ] Search through categories (using TSVECTOR of name + description)
  - [ ] Search through todos (using TSVECTOR of name + description)
  - [ ] Completed/incomplete filter/modifier for searching todos
  - [ ] Completed/incomplete filter/modifier for getting all todos or todos under category
- [ ] \*Nested categories
- [ ] \*Collaboration features:
  - [ ] Shared categories
  - [ ] Commenting on categories/todos
  - [ ] Stages of completion for todos
  - [ ] Friending system

##### \* = probably not going to be implemented

## Using:

- PostgreSQL - The objectively best database
- [Actix Web](https://actix.rs/) - An extremely fast, feature rich web framework for rust
- [SQLx](https://github.com/launchbadge/sqlx) - An asynchronous, type safe SQL toolkit
- [Argon2](https://docs.rs/argon2) - Pure rust argon2 password hashing

## Routes:

- `/api`
  - `/user`
    - `PUT` - Update user details
    - `/auth`
      - `POST /register` - Creates user
      - `POST /login` - Takes in username + email + password and returns refresh token + access token
      - `GET /refresh` - Takes in refresh token and returns new access token
  - `/categories` - All require access token
    - `GET ?limit=<limit>` - Get all categories for user
    - `POST` - Create category
    - `GET /search?query=<query>&limit=<limit>` - Search through categories
    - `/{cat_id}`
      - `GET ?filter=<none|completed|incomplete>&limit=<limit>` - Get category for user (with child todos joined)
      - `PUT` - Update category details
      - `DELETE` - Delete category
      - `/todos`
        - `GET ?filter=<none|completed|incomplete>&limit=<limit>` - Get todos under category
        - `GET /search?query=<query>&filter=<none|completed|incomplete>&limit=<limit>` - Search todos within category
        - `POST` - Create todo under category
        - `/{todo_id}`
          - `GET` - Get todo with all information
          - `PUT /toggle` - Toggle completedness of todo
          - `PUT` - Update details of todo
          - `DELETE` - Delete todo
  - `/todos`
    - `GET ?filter=<none|completed|incomplete>&limit=<limit>` - Get all todos for user
    - `GET /search?query=<query>&filter=<none|completed|incomplete>&limit=<limit>` - Search all todos for user

## Explanation:

### `PUT /api/user`

Authentication: "refresh_token" cookie,  
Description: Updates a user's details,  
Example Request Body:

```json
{
  "displayname": "John Doe",
  "username": "johnd03",
  "bio": "Hello I am John"
}
```

### `POST /api/user/register`

Authentication: none,  
Description: Creates a user,  
Example Request Body:

```json
{
  "displayname": "John Doe",
  "username": "johnd03",
  "email": "john.doe@example.com",
  "password": "$Pa55w0rd!"
}
```

### `POST /api/user/login`

Authentication: login details in request body,  
Description: Logs user in (sends back "refresh_token" and "access_token" cookies)  
Example Request Body:

```json
{
  "username": "johnd03",
  "email": "john.doe@example.com",
  "password": "$Pa55w0rd!"
}
```

### `GET /api/categories?limit=<limit>`

Authentication: "access_token" cookie,  
Description: Gets all categories for user,  
Query Parameters: `limit` (optional) is the maximum number of categories to be returned,  
Example Request: `GET /api/categories?limit=3`,  
Example Response Body:

```jsonc
[
  {
    "id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "name": "Health and Fitness",
    "description": "Dieting and Exercising goals",
    "created_at": "1970-01-01T00:00:00.000Z", // ISO 8601
    "edited_at": "1970-01-01T00:00:01.000Z" // ISO 8601
  },
  {
    "id": "1ef661e7-8c5c-486d-8a8c-a09c4ef896c5",
    "name": "School",
    "description": "Homework, assignments, exam dates etc",
    "created_at": "1970-01-01T00:00:02.000Z",
    "edited_at": "1970-01-01T00:00:03.000Z"
  },
  {
    "id": "a86292f8-d33f-4b6f-9390-076c93634fc0",
    "name": "Work",
    "description": "Work things",
    "created_at": "1970-01-01T00:00:04.000Z",
    "edited_at": "1970-01-01T00:00:05.000Z"
  }
]
```

### `POST /api/categories`

Authentication: "access_token" cookie,  
Description: Creates a category,  
Example Request Body:

```jsonc
{
  "name": "Category name",
  "description": "This is a description of the category, bla bla bla, yada yada yada"
  // timestamps will be auto-generated by the API
}
```

### `GET /api/categories/search?query=<query>&limit=<limit>`

Authentication: "access_token" cookie,  
Description: Searches through categories,  
Example Request: `GET /api/categories/search?query=and&limit=3`,  
Example response:

```jsonc
[
  {
    "id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "name": "Health and Fitness",
    "description": "Dieting and Exercising goals",
    "created_at": "1970-01-01T00:00:00.000Z", // ISO 8601
    "edited_at": "1970-01-01T00:00:01.000Z" // ISO 8601
  },
  {
    "id": "1ef661e7-8c5c-486d-8a8c-a09c4ef896c5",
    "name": "School",
    "description": "Homework, assignments and exam dates etc",
    "created_at": "1970-01-01T00:00:02.000Z",
    "edited_at": "1970-01-01T00:00:03.000Z"
  },
  {
    "id": "a86292f8-d33f-4b6f-9390-076c93634fc0",
    "name": "Work",
    "description": "Work and things",
    "created_at": "1970-01-01T00:00:04.000Z",
    "edited_at": "1970-01-01T00:00:05.000Z"
  }
]
```

### `GET /api/categories/{cat_id}?filter=<none|completed|incomplete>&limit=<limit>`

Authentication: "access_token" cookie,  
Description: Gets single category with id of `cat_id` and joins all child todos within,  
Query Parameters:

- `filter` (required) is applied to all of the child todos
- `limit` (optional) is the maximum number of child todos

Example Request: `GET /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810?filter=completed&limit=3`  
Example Response Body:

```jsonc
{
  "id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
  "name": "Health and Fitness",
  "description": "Dieting and Exercising goals",
  "todos": [
    {
      "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
      "id": "b2f1870d-4596-419f-ab35-6ac63fc94822",
      "title": "Cruciferous vegetables night",
      "description": "Have cruciferous vegetables for dinner",
      "completed": true,
      "created_at": "1970-01-01T00:00:02.000Z",
      "edited_at": "1970-01-01T00:00:02.000Z"
    },
    {
      "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
      "id": "5d7760c0-1406-4c09-a167-f4517424fc2d",
      "title": "Exercise",
      "description": "Go on bike ride around town",
      "completed": true,
      "created_at": "1970-01-01T00:00:03.000Z",
      "edited_at": "1970-01-01T00:00:03.000Z"
    },
    {
      "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
      "id": "477fbb73-97f5-4040-834c-df9de0eb110b",
      "title": "Sleep",
      "description": "Actually sleep",
      "completed": true,
      "created_at": "1970-01-01T00:00:04.000Z",
      "edited_at": "1970-01-01T00:00:04.000Z"
    }
  ],
  "created_at": "1970-01-01T00:00:00.000Z",
  "updated_at": "1970-01-01T00:00:04.000Z"
},
```

### `PUT /api/categories/{cat_id}`

Authentication: "access_token" cookie,  
Description: Updates category with id `cat_id`,  
Example Request Body:

```jsonc
{
  "name": "updated",
  "description": "this category has been updated"
  // edited_at will be automatically set
}
```

### `DELETE /api/categories/{cat_id}`

Authentication: "access_token" cookie,  
Description: Deletes category with id `cat_id`,  
Example Request: `DELETE /api/categories/4a3e7913-8eb1-467f-8903-ce8393cdcbd5`

### `GET /api/categories/{cat_id}/todos?filter=<none|completed|incomplete>&limit=<limit>`

Authentication: "access_token" cookie,  
Description: Gets all todos under category `cat_id`,  
Example Request: `GET /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810/todos?filter=completed&limit=3`
Example Response Body:
```jsonc
[
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "b2f1870d-4596-419f-ab35-6ac63fc94822",
    "title": "Cruciferous vegetables night",
    "description": "Have cruciferous vegetables for dinner",
    "completed": true,
    "created_at": "1970-01-01T00:00:02.000Z",
    "edited_at": "1970-01-01T00:00:02.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "5d7760c0-1406-4c09-a167-f4517424fc2d",
    "title": "Exercise",
    "description": "Go on bike ride around town",
    "completed": true,
    "created_at": "1970-01-01T00:00:03.000Z",
    "edited_at": "1970-01-01T00:00:03.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "477fbb73-97f5-4040-834c-df9de0eb110b",
    "title": "Sleep",
    "description": "Actually sleep",
    "completed": true,
    "created_at": "1970-01-01T00:00:04.000Z",
    "edited_at": "1970-01-01T00:00:04.000Z"
  }
]
```

### `GET /api/categories/{cat_id}/todos/search?query=<query>&filter=<none|completed|incomplete>&limit=<limit>`

Authentication: "access_token" cookie,  
Description: Searches for todos under category `cat_id`,  
Query Parameters:
- `query` (required) search query
- `filter` (required) applied to todos
- `limit` (optional) maximum number of todos to be returned

Example Request: `GET /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810/todos/search?query=and&filter=incomplete&limit=3`
Example Response Body:

```jsonc
[
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "b6809348-3623-4ddc-b87c-d759e9fc410d",
    "title": "Cheese and beans",
    "description": "cheese and beans",
    "completed": true,
    "created_at": "1970-01-01T00:00:05.000Z",
    "edited_at": "1970-01-01T00:00:05.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "dac145e3-e81c-4f9c-9819-eb8846c283d5",
    "title": "Exercise",
    "description": "Do streches and go on bike ride",
    "completed": true,
    "created_at": "1970-01-01T00:00:06.000Z",
    "edited_at": "1970-01-01T00:00:06.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "5c09a02a-deb6-4505-964c-c4f3621251eb",
    "title": "Sleep",
    "description": "Drink warm milk and sleep early",
    "completed": true,
    "created_at": "1970-01-01T00:00:07.000Z",
    "edited_at": "1970-01-01T00:00:07.000Z"
  }
]
```

### `POST /api/categories/{cat_id}/todos`

Authentication: "access_token" cookies,  
Description: Creates todo under category `cat_id`,  
Example Request Body:

```jsonc
// POST /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810/todos
{
  // cat_id will automatically be inserted
  "name": "Meditate",
  "description": "Meditate for an hour in the morning",
  "completed": false
  // timestamps will be automatically generated
}
```

### `GET /api/categories/{cat_id}/todos/{todo_id}`

Authentication: "access_token" cookie,  
Description: Get todo with id of `todo_id` under category `cat_id`,  
Example Request: `GET /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810/todos/b6809348-3623-4ddc-b87c-d759e9fc410d`
Example Response Body:

```jsonc
{
  "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
  "id": "b6809348-3623-4ddc-b87c-d759e9fc410d",
  "title": "Cheese and beans",
  "description": "cheese and beans",
  "completed": true,
  "created_at": "1970-01-01T00:00:05.000Z",
  "edited_at": "1970-01-01T00:00:10.000Z"
}
```

### `PUT /api/categories/{cat_id}/todos/{todo_id}/toggle`

Authentication: "access_token" cookie,  
Description: Toggles `completed` on todo with id of `todo_id` under category `cat_id`,  
Example Request: `PUT /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810/todos/b6809348-3623-4ddc-b87c-d759e9fc410d/toggle`

### `PUT /api/categories/{cat_id}/todos/{todo_id}`

Authentication: "access_token" cookie,  
Description: Updates todo with id `todo_id` under category `cat_id`,  
Example request:

```jsonc
{

  "title": "Updated",
  "description": "this has been updated",
  "completed": true,
  // edited_at will be automatically set
}
```

### `DELETE /api/categories/{cat_id}/todos/{todo_id}`

Authentication: "access_token" cookie,  
Description: Deletes todo with id of `todo_id` under category `cat_id`,  
Example Request: `DELETE /api/categories/c3627f3b-7d51-4905-b3a3-553fb5b90810/todos/b6809348-3623-4ddc-b87c-d759e9fc410d`

### `GET /api/todos?filter=<none|completed|incomplete>&limit=<limit>`

Authentication: "access_token" cookie,
Description: Gets all todos (regardless of category),
Query Parameters:
- `filter` (required) applied to the todos
- `limit` (optional) maximum number of todos

Example Request: `GET /api/todos?filter=none&limit=3`
Example Response Body:
```jsonc
[
  {
    "cat_id": "96756e2b-4ede-4971-9056-fa89800fc867",
    "id": "4529b4ef-958e-4424-8642-bfcb25021ca8",
    "title": "finish assignments",
    "description": "finish homework sheets and essay",
    "completed": false,
    "created_at": "1970-01-01T00:00:11.000Z",
    "edited_at": "1970-01-01T00:00:11.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "dac145e3-e81c-4f9c-9819-eb8846c283d5",
    "title": "Exercise",
    "description": "Do streches and go on bike ride",
    "completed": true,
    "created_at": "1970-01-01T00:00:06.000Z",
    "edited_at": "1970-01-01T00:00:06.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "5c09a02a-deb6-4505-964c-c4f3621251eb",
    "title": "Sleep",
    "description": "Drink warm milk and sleep early",
    "completed": true,
    "created_at": "1970-01-01T00:00:07.000Z",
    "edited_at": "1970-01-01T00:00:07.000Z"
  }
]
```

### `GET /api/todos/search?query=<query>&filter=<none|completed|incomplete>&limit=<limit>`

Authentication: "access_token" cookie,  
Description: Searches through all of a user's todos,  
Query Parameters:
- `query` (required) the search query
- `filter` (required) applied to todos
- `limit` (optional) maximum number of todos to be returned

Example Request: `GET /api/todos/search?query=and&filter=none&limit=4`
Example Response Body:

```jsonc
[
  {
    "cat_id": "96756e2b-4ede-4971-9056-fa89800fc867",
    "id": "4529b4ef-958e-4424-8642-bfcb25021ca8",
    "title": "finish assignments",
    "description": "finish homework sheets and essay",
    "completed": false,
    "created_at": "1970-01-01T00:00:11.000Z",
    "edited_at": "1970-01-01T00:00:11.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "dac145e3-e81c-4f9c-9819-eb8846c283d5",
    "title": "Exercise",
    "description": "Do streches and go on bike ride",
    "completed": true,
    "created_at": "1970-01-01T00:00:06.000Z",
    "edited_at": "1970-01-01T00:00:06.000Z"
  },
  {
    "cat_id": "c3627f3b-7d51-4905-b3a3-553fb5b90810",
    "id": "5c09a02a-deb6-4505-964c-c4f3621251eb",
    "title": "Sleep",
    "description": "Drink warm milk and sleep early",
    "completed": true,
    "created_at": "1970-01-01T00:00:07.000Z",
    "edited_at": "1970-01-01T00:00:07.000Z"
  }
]
```

##### PLEASE DO NOT USE THIS IN PRODUCTION, THERE ARE PROBABLY A _LOT_ OF SECURITY FLAWS

## Why? [Essay incoming :)]

This project is part of a journey spanning months to find a language that I genuinely enjoy using for backend, it started off in nodejs which was actually quite nice, it was very easy to get started and write things that work but I soon realised was not really suitable for much beyond prototyping IMO especially with the dynamic typing and it being an interpreted language. I then decided to try out golang which I thoroughly enjoyed, it had many things I appreciated e.g. simple syntax, very easy to pick up the basics, the speed (as compared to nodejs) explicit error handling, the fast compilation, the list goes on **but** there also were many gripes I had with it, e.g. the lack of generics and therefore dynamic typing in the form of `interface{}`, the inconsistent syntax, the seemingly unnecessarily large binaries, the bad windows compatibility, the "community" etc.

I decided I would give rust a try and consider switching over from golang, so I decided to start by reading _the book_ until I made it to the smart pointers chapter, when I realised that I should probably get started writing some code, I had already done several very small projects following the book and I really liked the syntax so I then decided to loosely follow a tutorial and write a shell, this made me realise how much better the std lib abstractions were in rust than in go and how amazing rust's pattern matching is.

Usually I end up getting started by jumping into the deep end too early so in rust I decided to incrementally make my way to this project, so I started out by making an [in-memory todolist API with warp](https://github.com/jawadcode/basic-todoapi) and it made me appreciate how well rust dealt with thread safety (although I used `parking_lot` over `std::sync`), how traits are integral to rust, how amazing the iterator methods are e.g. map and filter and also how helpful cargo is.

Although, I had heard that actix was the preferred web framework in rust so I decided to give it a try by making a copy of the warp API [with actix](https://github.com/jawadcode/basic-todoapi-actix) (using `std::sync` this time) and I realised my preconcieved notion of actix being too complex was wrong as it was just as easy if not easier in actix than in warp.

SQLx was next on my list which I learnt by remaking the previous (actix) project but [using an actually persistent database](https://github.com/jawadcode/basic-todoapi-actix-sqlx) (with the help of [this example](https://github.com/actix/examples/tree/master/sqlx_todo)) one thing that was really helpful was that the macros could connect to the DB and ensure the queries were type safe. Now that I knew: the language, a web framework, and a database toolkit, I decided it was time to jump into the big project.

TL;DR Rust is the best language I have ever used.
# Rest api routes

### GET /player/id/{id}
- Success: [Player](entities.md#player)
- Not exist: 404

### GET /player/name/{name}
- Success: [Player](entities.md#player)
- Not exist: 404

### GET /application/{last_id}
- Success: [Application Page](entities.md#application-page) (possibly empty)
- Authentication
- Requester's role is not mod: 403

### PATCH /application/accept/{id}
- Success: 204
- Authentication
- Requester's role is not mod: 403

### PATCH /application/decline/{id}
- Body: [Application Decline Reason](entities.md#application-decline-reason)
- Success: 204
- Authentication
- Requester's role is not mod: 403

### GET /application
- Success: [Application](entities.md#application)
- Authentication
- Not exist: 404

### POST /application
- Success: 201
- Authentication
- Could be rewritten

### DELETE /application
- Success: 204
- Authentication
- Not exist: 404

# Authentication
- Should be done with header 'authentication' with value of: 'token {token}'
- If Authentication fails then 403 status code sends
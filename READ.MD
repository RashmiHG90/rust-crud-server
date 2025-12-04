
cargo run


# Create a task
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "description": "Build a CRUD API"}'

# Get all tasks
curl http://localhost:3000/tasks

# Update a task (replace {id} with actual UUID)
curl -X PUT http://localhost:3000/tasks/{id} \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

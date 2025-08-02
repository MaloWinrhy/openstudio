#!/bin/bash
curl -X POST http://localhost:3000/projects \
  -H 'Content-Type: application/json' \
  -d '{"name":"Demo Project","description":"Un projet de test"}'

echo -e "\n---\n"
curl http://localhost:3000/projects

echo -e "\n---\n"
# curl http://localhost:3000/projects/<id>

echo -e "\n---\n"
# curl -X PUT http://localhost:3000/projects/<id> \
#   -H 'Content-Type: application/json' \
#   -d '{"name":"Nouveau nom"}'

echo -e "\n---\n"
# curl -X DELETE http://localhost:3000/projects/<id>

echo -e "\n---\n"
curl -X POST http://localhost:3000/issues \
  -H 'Content-Type: application/json' \
  -d '{"project_id":"<project_id>","title":"Bug critique","description":"Description du bug"}'

echo -e "\n---\n"
# Lister les issues d'un projet (remplacer <project_id>)
curl "http://localhost:3000/issues?project_id=<project_id>"

echo -e "\n---\n"
# curl http://localhost:3000/issues/<id>

echo -e "\n---\n"
# curl -X PUT http://localhost:3000/issues/<id> \
#   -H 'Content-Type: application/json' \
#   -d '{"status":"Closed"}'

echo -e "\n---\n"
# curl -X DELETE http://localhost:3000/issues/<id>

#!/bin/bash
# Créer un projet
curl -X POST http://localhost:3000/projects \
  -H 'Content-Type: application/json' \
  -d '{"name":"Demo Project","description":"Un projet de test"}'

echo -e "\n---\n"
# Lister les projets
curl http://localhost:3000/projects

echo -e "\n---\n"
# Récupérer un projet par ID (remplacer <id>)
# curl http://localhost:3000/projects/<id>

echo -e "\n---\n"
# Mettre à jour un projet (remplacer <id>)
# curl -X PUT http://localhost:3000/projects/<id> \
#   -H 'Content-Type: application/json' \
#   -d '{"name":"Nouveau nom"}'

echo -e "\n---\n"
# Supprimer un projet (remplacer <id>)
# curl -X DELETE http://localhost:3000/projects/<id>

echo -e "\n---\n"
# Créer une issue (remplacer <project_id>)
curl -X POST http://localhost:3000/issues \
  -H 'Content-Type: application/json' \
  -d '{"project_id":"<project_id>","title":"Bug critique","description":"Description du bug"}'

echo -e "\n---\n"
# Lister les issues d'un projet (remplacer <project_id>)
curl "http://localhost:3000/issues?project_id=<project_id>"

echo -e "\n---\n"
# Récupérer une issue par ID (remplacer <id>)
# curl http://localhost:3000/issues/<id>

echo -e "\n---\n"
# Mettre à jour une issue (remplacer <id>)
# curl -X PUT http://localhost:3000/issues/<id> \
#   -H 'Content-Type: application/json' \
#   -d '{"status":"Closed"}'

echo -e "\n---\n"
# Supprimer une issue (remplacer <id>)
# curl -X DELETE http://localhost:3000/issues/<id>

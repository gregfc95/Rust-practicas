#!/bin/sh
COMMIT_MSG=$(cat "$1")

REGEX="^(feat|fix|docs|style|refactor|test|chore)(\([a-z0-9-]+\))?: .+"

# Validar
if ! echo "$COMMIT_MSG" | grep -Eq "$REGEX"; then
  echo "❌ Error: Formato de commit inválido."
  echo "Ejemplo válido: feat: add ejercicio 3"
  echo "Tipos permitidos: feat, fix, docs, style, refactor, test, chore"
  exit 1
fi

exit 0
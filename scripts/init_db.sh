#!/usr/bin/env bash
#la linea anterior es un "shebang" que indica que el script debe ejecutarse con bash

# Habilita el modo de depuración (muestra los comandos que se ejecutan)
set -x

# Detiene el script si ocurre cualquier error o si fall#!/usr/bin/env bash
#la linea anterior es un "shebang" que indica que el script debe ejecutarse con bash

# Habilita el modo de depuración (muestra los comandos que se ejecutan)
set -x

# Detiene el script si ocurre cualquier error o si falla un pipe
set -eo pipefail

# Variables con valores por defecto si no se definen externamente
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Verifica si ya existe un contenedor con el mismo nombre
if [ "$(docker ps -aq -f name=newsletter-db)" ]; then
  echo "Ya existe un contenedor llamado newsletter-db. Detenlo antes de ejecutar este script."
  exit 1
fi

#verifica si las herramientas necesarias están instaladas
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "  cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

if [[ -z "${SKIP_DOCKER}" ]]
then
	# Lanza el contenedor de PostgreSQL
	docker run \
		--name newsletter-db \
		-e POSTGRES_USER="${DB_USER}" \
		-e POSTGRES_PASSWORD="${DB_PASSWORD}" \
		-e POSTGRES_DB="${DB_NAME}" \
		-p "${DB_PORT}":5432 \
		-d postgres:14 \
		-c 'max_connections=1000'
fi
# Espera a que PostgreSQL esté listo para aceptar conexiones
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' 2>/dev/null; do
  >&2 echo "Postgres aún no está disponible... esperando"
  sleep 1
done

>&2 echo "✅ Postgres está listo y corriendo en el puerto ${DB_PORT}"

# Exporta la URL de conexión para sqlx y otros componentes
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run # Ejecuta las migraciones pendientes

>&2 echo "✅ Base de datos ${DB_NAME} creada y migraciones aplicadas"

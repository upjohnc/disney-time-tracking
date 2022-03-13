default:
    just --list

add_time type:
    poetry run python src/tracking.py insert-time {{type}}

show:
    poetry run python src/tracking.py show-file

calculate:
    poetry run python src/tracking.py calculate-time

check:
    poetry run python src/tracking.py check-file

docker_build:
    docker compose -f ./docker/docker-compose.yml build tracker

docker_run:
    docker compose -f ./docker/docker-compose.yml run --rm tracker

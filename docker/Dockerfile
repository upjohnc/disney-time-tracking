FROM python:3.10-slim-buster AS base

ENV PYTHONPATH=/app/src

RUN pip install --no-cache-dir -U pip poetry


FROM base AS application

COPY ./src /app/src
COPY ./poetry.lock /app/
COPY ./pyproject.toml /app/

WORKDIR /app

RUN poetry install --no-dev


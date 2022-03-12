import copy
import datetime as dt
from pathlib import Path
from typing import Dict, List, Tuple

import click
from yaml import Loader, dump, load


def root_dir():
    present_dir = Path(".")
    root_dir = present_dir.parent.resolve()
    return root_dir


def file_path():
    file_path = Path("data/tracking.yml")
    full_path = root_dir() / file_path
    return full_path


def retrieve_file():
    with open(file_path()) as f:
        t = load(f, Loader=Loader)
    return t


def save_data_back_to_file(data: Dict) -> None:
    with open(file_path(), "w") as f:
        dump(data, f)


def get_datetime_now() -> Tuple[str, str]:
    datetime_now = dt.datetime.utcnow()
    date_str = datetime_now.strftime("%Y-%m-%d")
    utc_time = datetime_now.strftime("%H:%M")
    return date_str, utc_time


def create_time_entry(time_type: str, utc_time_str: str) -> List:
    time_entry = [time_type, utc_time_str]
    return time_entry


@click.command()
@click.argument("time_type")
def insert_time(time_type: str):
    time_type = time_type.lower()
    options = ["start", "stop"]
    if time_type not in options:
        raise Exception(f"Type needs to be in {options}")

    date_str, utc_time_str = get_datetime_now()

    present_data = retrieve_file()
    data = copy.deepcopy(present_data) if present_data is not None else {}

    time_entry = create_time_entry(time_type, utc_time_str)
    if date_str not in data.keys():
        data[date_str] = []
    existing_time_entries = data[date_str]
    existing_time_entries.append(time_entry)
    _ = save_data_back_to_file(data)

    print(data)


@click.group()
def group():
    pass


group.add_command(insert_time)

if __name__ == "__main__":
    # print(retrieve_file())

    group()

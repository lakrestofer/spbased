#!/usr/bin/env python
import click
import os
import shutil
import sys
import typer
from typing_extensions import Annotated

BANNER = """\b
       _____       __                        __   ________           __                        __
      / ___/____  / /_  ____ _________  ____/ /  / ____/ /___ ______/ /_  _________ __________/ /____
      \\__ \\/ __ \\/ __ \\/ __ `/ ___/ _ \\/ __  /  / /_  / / __ `/ ___/ __ \\/ ___/ __ `/ ___/ __  / ___/
     ___/ / /_/ / /_/ / /_/ (__  )  __/ /_/ /  / __/ / / /_/ (__  ) / / / /__/ /_/ / /  / /_/ (__  )
    /____/ .___/_.___/\\__,_/____/\\___/\\__,_/  /_/   /_/\\__,_/____/_/ /_/\\___/\\__,_/_/   \\__,_/____/
        /_/
    Content agnostic spaced repetition
    """

app = typer.Typer(help=BANNER)


def spbased_cli():
    flake_root = os.environ.get("FLAKE_ROOT")
    bin = "spbased" if (flake_root is None) else f"{flake_root}/target/debug/spbasedctl"
    exists = (shutil.which(bin) is not None) or (
        os.path.isfile(bin) and os.access(bin, os.X_OK)
    )
    if not exists:
        click.echo("Could not find spbased on $PATH", err=True)
        sys.exit(1)
    return bin


@app.command()
def add(question: Annotated[str, typer.Option(prompt="Question")], answer: Annotated[str, typer.Option(prompt=True)]):
    """Create a new flashcard"""
    click.echo("hello there")


@app.command()
def edit(id: int):
    """Edit a flashcard"""
    click.echo(f"hello there {id}")


@app.command()
def review():
    """Review the next due or new review item"""
    click.echo(f"hello there {id}")


if __name__ == "__main__":
    app()

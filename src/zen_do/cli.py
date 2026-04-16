from cyclopts import App

from zen_do.get_token import get_token
from zen_do.zenodo import (
    zenodo_create_deposit,
    zenodo_get_deposit,
    zenodo_update_deposit,
)

app = App(
    name="zen-do",
    help=(
        "The `zen_do` package contains GitHub reusable workflows and "
        "actions used in the Seedcase Project."
    ),
)


@app.command()
def zenodo_publish(sandbox: bool = False) -> None:
    """Publish a new version of the repository on Zenodo."""
    token = get_token(sandbox)
    if deposit := zenodo_get_deposit(token):
        zenodo_update_deposit(token, deposit.id)
        print("Zenodo deposit updated successfully!")
    else:
        zenodo_create_deposit(token)
        print("New Zenodo deposit created successfully!")

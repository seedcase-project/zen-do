from cyclopts import App

from zen_do.get_token import get_token
from zen_do.zenodo import (
    zenodo_create_record,
    zenodo_get_record,
    zenodo_update_record,
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
    if record := zenodo_get_record(token):
        zenodo_update_record(token, record.id)
        print("Zenodo record updated successfully!")
    else:
        zenodo_create_record(token)
        print("New Zenodo record created successfully!")

from cyclopts import App

from zen_do.get_token import get_token
from zen_do.zenodo import (
    zenodo_get_record,
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
        print("Zenodo record updated successfully!")
        # To appease linter.
        print(record)
    else:
        print("New Zenodo record created successfully!")

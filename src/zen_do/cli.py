from seedcase_soil import (
    # print_if_verbose,
    run_without_tracebacks,
    setup_cli,
)

from zen_do.get_token import get_token
from zen_do.zenodo import ZenodoClient, zenodo_get_deposit

app = setup_cli(
    name="zen-do",
    help="zen-do simplifies interacting with Zenodo for common publishing tasks.",
)


@app.command()
def zenodo_publish(sandbox: bool = False) -> None:
    """Publish a new version of the repository on Zenodo."""
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    if deposit := zenodo_get_deposit(client.get_deposits()):
        print("Zenodo deposit updated successfully!")
        print(f"{deposit}")
    else:
        print("New Zenodo record created successfully!")


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    run_without_tracebacks(app)

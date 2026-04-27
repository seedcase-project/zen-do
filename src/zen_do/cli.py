import seedcase_soil as so
from rich import print_json

from zen_do.get_token import get_token
from zen_do.zenodo_client import (
    ZenodoClient,
)

app = so.setup_cli(
    name="zen-do",
    help="zen-do simplifies interacting with Zenodo for common publishing tasks.",
)


@app.command()
def list(sandbox: bool = False) -> None:
    """List all Zenodo deposits in an account as raw JSON (from the Zenodo servers).

    Args:
        sandbox: Whether to use the Zenodo sandbox environment for testing purposes.
    """
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    deposits = client.get_deposits()

    if deposits:
        print_json(data=deposits)
    else:
        so.pretty_print("No deposits found.")


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    so.run_without_tracebacks(app)

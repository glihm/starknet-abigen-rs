# SN ABI

A simple CLI tool to fetch/extract ABIs.

## Fetch an ABI

To fetch an abi, enter `snabi fetch --help`.

Providing the URL of the node and the address of the contract you will be able to get the ABI downloaded.

## Extract from Sierra

If you are working locally, you can use `snabi from-sierra --help` to see how providing a path to a Sierra file you can generate the corresponding rust code.

## Outputs

You can output the ABI into a plain JSON, using the `--json` option. Or you can decide to directly generate a rust module that can be imported directly into rust code with the `--expandable` option. This generated module will expand the ABI into corresponding rust types.

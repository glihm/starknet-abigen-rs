config := --account data/katana_account.json \
	--keystore data/katana_key.json \
	--rpc http://0.0.0.0:5050 \
	--keystore-password 1234

class_hash = $(shell starkli class-hash target/dev/contracts_c1.sierra.json)

# Declare and deploy the contract on katana.
setup:
	starkli declare target/dev/contracts_c1.sierra.json ${config}
	starkli deploy ${class_hash} --salt 0x1234 ${config}

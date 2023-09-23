config := --account data/katana_account.json \
	--keystore data/katana_key.json \
	--rpc http://0.0.0.0:5050 \
	--keystore-password 1234

# Declare and deploy the basic contract on katana.
setup_basic:
	$(eval class_hash=$(shell starkli class-hash target/dev/contracts_basic.sierra.json))
	starkli declare target/dev/contracts_basic.sierra.json ${config}
	starkli deploy ${class_hash} --salt 0x1234 ${config}

# Declare and deploy the basic contract on katana.
setup_gen:
	$(eval class_hash=$(shell starkli class-hash target/dev/contracts_gen.sierra.json))
	starkli declare target/dev/contracts_gen.sierra.json ${config}
	starkli deploy ${class_hash} --salt 0x1234 ${config}


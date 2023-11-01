config := --account katana-0 \
	--rpc http://0.0.0.0:5050

setup: setup_simple_get_set

# Declare and deploy the simple_get_set contract on katana.
setup_simple_get_set:
	$(eval class_hash=$(shell starkli class-hash target/dev/contracts_simple_get_set.contract_class.json))
	starkli declare target/dev/contracts_simple_get_set.contract_class.json ${config}
	starkli deploy ${class_hash} --salt 0x1234 ${config}

# # Declare and deploy the basic contract on katana.
# setup_basic:
# 	$(eval class_hash=$(shell starkli class-hash target/dev/contracts_basic.sierra.json))
# 	starkli declare target/dev/contracts_basic.sierra.json ${config}
# 	starkli deploy ${class_hash} --salt 0x1234 ${config}

# # Declare and deploy the basic contract on katana.
# setup_gen:
# 	$(eval class_hash=$(shell starkli class-hash target/dev/contracts_gen.sierra.json))
# 	starkli declare target/dev/contracts_gen.sierra.json ${config}
# 	starkli deploy ${class_hash} --salt 0x1234 ${config}

# # Declare and deploy the event contract on katana.
# setup_event:
# 	$(eval class_hash=$(shell starkli class-hash target/dev/contracts_event.sierra.json))
# 	starkli declare target/dev/contracts_event.sierra.json ${config}
# 	starkli deploy ${class_hash} --salt 0x1234 ${config}

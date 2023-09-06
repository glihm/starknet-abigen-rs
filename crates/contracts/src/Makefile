config := --account ~/screenshot/ArkProject/apps/arkchain/data/katana_account.json \
	--keystore ~/screenshot/ArkProject/apps/arkchain/data/katana_key.json \
	--keystore-password 1234

rpc := --rpc http://localhost:5050
class_hash = $(shell starkli class-hash target/dev/piaglihm_piaglihm.sierra.json)

setup:
	scarb build
	starkli declare target/dev/piaglihm_piaglihm.sierra.json ${config} ${rpc}
	starkli deploy ${class_hash} ${config} ${rpc} --salt 0x1234

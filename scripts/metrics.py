import requests

count = 0
vesting_count = 0
resp = requests.get('https://maximus.streamflow.finance/api/contracts/streamflow').json()
for contract in resp:
    ix = contract.get('ix')
    if not ix.get('can_topup'):
        vesting_count += 1
    else:
        continue
    if int(ix.get('cliff')) > int(ix.get('start_time')):
        count += 1

print(f"Number of contracts: {len(resp)}")


print(f"Number of vesting contracts: {vesting_count}")
print(f"Number of vesting contracts with cliff: {count}")
print(f"Percentage with cliff: {count / vesting_count}")


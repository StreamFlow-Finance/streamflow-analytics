def repr_contract_streamflow(contract, token):
    decimals = 0
    price = 0
    if token is not None:
        decimals = token.get('decimals') or 0
        price = token.get('price_usd') or 0

    contract['streamflow_fee_total'] = int(contract.get('streamflow_fee_total')) / 10**int(decimals)
    contract['streamflow_fee_withdrawn'] = int(contract.get('streamflow_fee_withdrawn')) / 10**int(decimals)
    contract['partner_fee_total'] = int(contract.get('partner_fee_total')) / 10**int(decimals)
    contract['partner_fee_withdrawn'] = int(contract.get('partner_fee_withdrawn')) / 10**int(decimals)
    contract['amount_withdrawn'] = int(contract.get('amount_withdrawn')) / 10**int(decimals)
    ix = contract.get('ix')
    ix['net_amount_deposited'] = int(ix.get('net_amount_deposited')) / 10**int(decimals)
    ix['amount_per_period'] = int(ix.get('amount_per_period')) / 10**int(decimals)
    ix['cliff_amount'] = int(ix.get('cliff_amount')) / 10**int(decimals)
    contract['total_value_usd'] = int(ix['net_amount_deposited']) * int(price)
    return contract


def repr_contract_community(contract, token):
    decimals = 0
    price = 0
    if token is not None:
        decimals = token.get('decimals') or 0
        price = token.get('price_usd') or 0
    contract['withdrawn_amount'] = int(contract.get('withdrawn_amount')) / 10**int(decimals)
    ix = contract.get('ix')
    ix['deposited_amount'] = int(ix.get('deposited_amount')) / 10**int(decimals)
    ix['cliff_amount'] = int(ix.get('cliff_amount')) / 10**int(decimals)
    ix['total_amount'] = int(ix.get('total_amount')) / 10**int(decimals)
    contract['total_value_usd'] = int(ix['total_amount']) * int(price)
    return contract
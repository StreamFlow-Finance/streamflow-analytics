import json
import redis
from flask import Blueprint, request
from datetime import timezone
import datetime

from process import filter
from repr import repr_contract_streamflow, repr_contract_community
from operations.state import RedisState

contracts_handler = Blueprint('contracts', __name__)


def get_contracts(program, representation):
    contracts = RedisState().get_contracts(program)
    contracts = filter(contracts, request.args)

    tokens = RedisState().get_tokens_metadata()
    for id, contract in contracts.items():
        token = tokens.get(contract.get('mint'))
        contracts[id] = representation(contract, token)

    resp_list = []
    for id, obj in contracts.items():
        obj['contract_address'] = id
        resp_list.append(obj)
    return json.dumps(resp_list)


@contracts_handler.route("/community")
def contracts_community():
    return get_contracts('community', repr_contract_community)


@contracts_handler.route("/streamflow")
def contracts_streamflow():
    return get_contracts('streamflow', repr_contract_streamflow)


@contracts_handler.route("/community/<address>")
def contracts_community_by_address(address):
    return RedisState().get_contracts('community').get(address)


@contracts_handler.route("/streamflow/<address>")
def contracts_streamflow_by_address(address):
    return RedisState().get_contracts('streamflow').get(address)


@contracts_handler.route("/community/summary")
def contracts_community_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    state = RedisState()
    contracts = state.get_contracts('community')
    tokens = state.get_tokens_data()

    # filter out no active streams
    total_active_streams = 0
    uq_tokens = {}
    total_value_sent = 0
    total_value_locked = 0

    for address, contract in contracts.items():
        mint = contract.get('mint')
        uq_tokens[mint] = 1
        token = tokens.get(mint)
        if token is None:
            continue
        if contract.get('ix').get('end_time') > now:
            total_active_streams += 1

            locked_value = (contract.get('ix').get('deposited_amount') - contract.get('withdrawn_amount')) / 10**int(token.get('decimals')) * float(token.get('price_usd'))
            total_value_locked += locked_value
        total_value_sent += contract.get('ix').get('deposited_amount') / 10**token.get('decimals') * token.get('price_usd')

    resp = {
        "total_streams_created": len(contracts.keys()),
        "active_streams": total_active_streams,
        "number_of_unique_tokens": len(uq_tokens.keys()),
        "total_value_locked": int(total_value_locked),
        "total_value_sent": int(total_value_sent)
    }
    return json.dumps(resp)


@contracts_handler.route("/streamflow/summary")
def contracts_streamflow_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    state = RedisState()
    contracts = state.get_contracts('streamflow')
    tokens = state.get_tokens_data()

    # filter out no active streams
    total_active_streams = 0
    uq_tokens = {}
    total_value_sent = 0
    total_value_locked = 0

    for address, contract in contracts.items():
        mint = contract.get('mint')
        uq_tokens[mint] = 1
        token = tokens.get(mint)
        if token is None:
            continue
        if contract.get('end_time') > now:
            total_active_streams += 1

            locked_value = (contract.get('ix').get('net_amount_deposited') - contract.get('amount_withdrawn')) / 10**int(token.get('decimals')) * float(token.get('price_usd'))
            total_value_locked += locked_value
        total_value_sent += contract.get('ix').get('net_amount_deposited') / 10**token.get('decimals') * token.get('price_usd')

    resp = {
        "total_streams_created": len(contracts.keys()),
        "active_streams": total_active_streams,
        "number_of_unique_tokens": len(uq_tokens.keys()),
        "total_value_locked": int(total_value_locked),
        "total_value_sent": int(total_value_sent)
    }
    return json.dumps(resp)


@contracts_handler.route("/summary")
def contracts_all_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    state = RedisState()
    community = state.get_contracts('community')
    streamflow = state.get_contracts('streamflow')
    tokens = state.get_tokens_data()

    # filter out no active streams
    total_active_streams = 0
    uq_tokens = {}
    total_value_sent = 0
    total_value_locked = 0

    for address, contract in streamflow.items():
        mint = contract.get('mint')
        uq_tokens[mint] = 1
        token = tokens.get(mint)
        if token is None:
            continue
        if contract.get('end_time') > now:
            total_active_streams += 1

            locked_value = (contract.get('ix').get('net_amount_deposited') - contract.get('amount_withdrawn')) / 10**int(token.get('decimals')) * float(token.get('price_usd'))
            total_value_locked += locked_value
        total_value_sent += contract.get('ix').get('net_amount_deposited') / 10**token.get('decimals') * token.get('price_usd')

    for address, contract in community.items():
        mint = contract.get('mint')
        uq_tokens[mint] = 1
        token = tokens.get(mint)
        if token is None:
            continue
        if contract.get('ix').get('end_time') > now:
            total_active_streams += 1

            locked_value = (contract.get('ix').get('deposited_amount') - contract.get('withdrawn_amount')) / 10**int(token.get('decimals')) * float(token.get('price_usd'))
            total_value_locked += locked_value
        total_value_sent += contract.get('ix').get('deposited_amount') / 10**token.get('decimals') * token.get('price_usd')

    resp = {
        "total_streams_created": len(community.keys()) + len(streamflow.keys()),
        "active_streams": total_active_streams,
        "number_of_unique_tokens": len(uq_tokens.keys()),
        "total_value_locked": int(total_value_locked),
        "total_value_sent": int(total_value_sent)
    }
    return json.dumps(resp)


import json
import redis
from flask import Blueprint, request
from datetime import timezone
import datetime

from process import filter
from repr import repr_contract_streamflow, repr_contract_community
from operations.tokens import RedisState

contracts_handler = Blueprint('contracts', __name__)


def get_contracts(query_set, representation):
    """

    :param query_set:
    :param representation: repr_contract_streamflow or repr_contract_community functions
    :return:
    """
    r = redis.Redis(host='localhost', port=6379)
    contracts = json.loads(r.get(query_set)).get('data')
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
    return get_contracts('contracts-community', repr_contract_community)


@contracts_handler.route("/streamflow")
def contracts_streamflow():
    return get_contracts('contracts-streamflow', repr_contract_streamflow)


@contracts_handler.route("/community/<address>")
def contracts_community_by_address(address):
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community'))
    return value.get('data').get(address)


@contracts_handler.route("/streamflow/<address>")
def contracts_streamflow_by_address(address):
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow'))
    return value.get('data').get(address)


@contracts_handler.route("/community/summary")
def contracts_community_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community')).get('data')
    tokens = RedisState().get_tokens_data()
    total_streams_created = len(value.keys())

    # filter out no active streams
    total_active_streams = 0
    uq_tokens = {}
    total_value_sent = 0
    total_value_locked = 0

    for address, contract in value.items():
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
        "total_streams_created": total_streams_created,
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

    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow')).get('data')
    tokens = RedisState().get_tokens_data()
    total_streams_created = len(value.keys())

    # filter out no active streams
    total_active_streams = 0
    uq_tokens = {}
    total_value_sent = 0
    total_value_locked = 0

    for address, contract in value.items():
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
        "total_streams_created": total_streams_created,
        "active_streams": total_active_streams,
        "number_of_unique_tokens": len(uq_tokens.keys()),
        "total_value_locked": int(total_value_locked),
        "total_value_sent": int(total_value_sent)
    }
    return json.dumps(resp)

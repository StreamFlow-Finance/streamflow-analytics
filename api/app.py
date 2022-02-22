from flask import Flask
import redis
import json
import os
from datetime import timezone
import datetime
import requests


application = Flask(__name__)


@application.route("/tokens")
def all_tokens():
    return get_tokens_data()


@application.route("/tokens/<address>")
def token_by_address(address):
    data = get_tokens_data()
    for mint, mint_data in data.items():
        if mint == address:
            return mint_data
    return {}


def get_tokens_data():
    r = redis.Redis(host='localhost', port=6379)
    metadata = requests.get('https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json').json().get('tokens')
    prices = json.loads(r.get('token-prices'))
    data = {}
    for mint, price in prices.items():
        for token in metadata:
            if token.get('address') == mint:
                token['price_usd'] = price
                data[mint] = token
    return data


@application.route("/contracts/community")
def contracts_community():
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community'))
    return value


@application.route("/contracts/streamflow")
def contracts_streamflow():
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow'))
    return value


@application.route("/contracts/community/<address>")
def contracts_community_by_address(address):
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community'))
    return value.get('data').get(address)


@application.route("/contracts/streamflow/<address>")
def contracts_streamflow_by_address(address):
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow'))
    return value.get('data').get(address)


@application.route("/contracts/community/summary")
def contracts_community_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community')).get('data')
    tokens = get_tokens_data()
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


@application.route("/contracts/streamflow/summary")
def contracts_streamflow_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow')).get('data')
    tokens = get_tokens_data()
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


@application.route("/tokens/streamflow/summary")
def tokens_streamflow_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow')).get('data')
    tokens = get_tokens_data()

    # filter out no active streams
    token_stats = {}

    for address, contract in value.items():
        mint = contract.get('mint')
        token = tokens.get(mint)
        if token is None:
            continue
        token['value'] = 0
        if token_stats.get(mint) is None:
            token_stats[mint] = token

        contract_value = contract.get('ix').get('net_amount_deposited') / 10**token.get('decimals') * token.get('price_usd')
        token['value'] += contract_value
    return token_stats


@application.route("/tokens/community/summary")
def tokens_community_summary():
    dt = datetime.datetime.now(timezone.utc)

    utc_time = dt.replace(tzinfo=timezone.utc)
    now = int(utc_time.timestamp())

    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community')).get('data')
    tokens = get_tokens_data()

    # filter out no active streams
    token_stats = {}

    for address, contract in value.items():
        mint = contract.get('mint')
        token = tokens.get(mint)
        if token is None:
            continue
        token['value'] = 0
        if token_stats.get(mint) is None:
            token_stats[mint] = token

        contract_value = contract.get('ix').get('deposited_amount') / 10**token.get('decimals') * token.get('price_usd')
        token['value'] += contract_value
    return token_stats


if __name__ == "__main__":
    port = int(os.environ.get("PORT", 5000))
    application.run(host='0.0.0.0', port=port, debug=True)

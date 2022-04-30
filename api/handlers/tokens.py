import json
import redis
from flask import Blueprint, request, current_app

from process import filter
from operations.state import RedisState

tokens_handler = Blueprint('tokens', __name__)


@tokens_handler.route("/")
def all_tokens():
    return RedisState().get_tokens_data()


@tokens_handler.route("/", methods=['POST'])
def add_token():
    token = request.headers.get('Auth-Token')
    if current_app.config.get('AUTH_TOKEN') != token or token is None:
        return {"Message": "Forbidden."}, 403
    body = request.json
    mint = body.get('mint')
    price = body.get('price')
    RedisState().add_token(mint, price)
    return {"Message": "OK"}


@tokens_handler.route("/<address>")
def token_by_address(address):
    data = RedisState().get_tokens_data()
    for mint, mint_data in data.items():
        if mint == address:
            return mint_data
    return {}


@tokens_handler.route("/streamflow/summary")
def tokens_streamflow_summary():
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-streamflow'))
    c = filter(value.get('data'), request.args)
    value['data'] = c
    tokens = RedisState().get_tokens_metadata()

    # filter out no active streams
    token_stats = {}

    for address, contract in value.get('data').items():
        mint = contract.get('mint')
        token = tokens.get(mint)
        if token is None:
            continue
        if token.get('value') is None:
            token['value'] = 0
        if token.get('contracts_count') is None:
            token['contracts_count'] = 0
        if token_stats.get(mint) is None:
            token_stats[mint] = token

        contract_value = contract.get('ix').get('net_amount_deposited') / 10**token.get('decimals') * token.get('price_usd')
        token['value'] += contract_value
        token['contracts_count'] += 1

    return json.dumps(list(token_stats.values()))


@tokens_handler.route("/community/summary")
def tokens_community_summary():
    r = redis.Redis(host='localhost', port=6379)
    value = json.loads(r.get('contracts-community'))
    c = filter(value.get('data'), request.args)
    value['data'] = c
    tokens = RedisState().get_tokens_metadata()

    # filter out no active streams
    token_stats = {}

    for address, contract in value.get('data').items():
        mint = contract.get('mint')
        token = tokens.get(mint)
        if token is None:
            continue
        if token.get('value') is None:
            token['value'] = 0
        if token.get('contracts_count') is None:
            token['contracts_count'] = 0
        if token_stats.get(mint) is None:
            token_stats[mint] = token
        contract_value = contract.get('ix').get('deposited_amount') / 10**token.get('decimals') * token.get('price_usd')
        token['value'] += contract_value
        token['contracts_count'] += 1
    return json.dumps(list(token_stats.values()))

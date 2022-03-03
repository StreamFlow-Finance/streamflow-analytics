import redis
import requests
import json


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


def get_tokens_metadata():
    r = redis.Redis(host='localhost', port=6379)
    metadata = requests.get('https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json').json().get('tokens')
    data = {}
    prices = json.loads(r.get('token-prices'))
    for token in metadata:
        mint = token.get('address')
        price = prices.get(mint)
        if price is None:
            price = 0
        token['price_usd'] = price
        data[mint] = token
    return data
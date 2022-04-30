import redis
import requests
import json


custom_token_prices = 'custom-token-prices'
token_list_url = 'https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json'

program_keys = {
    'streamflow': 'contracts-streamflow',
    'community': 'contracts-community'
}


class RedisState:

    def __init__(self):
        self.state = redis.Redis(host='localhost', port=6379)

    def get_contracts(self, program):
        json.loads(self.state.get(program_keys.get(program))).get('data')

    def get_token_prices(self):
        prices = json.loads(self.state.get(custom_token_prices))
        prices.update(json.loads(self.state.get('token-prices')))
        return prices

    @staticmethod
    def get_token_metadata():
        return requests.get(token_list_url).json().get('tokens')

    def add_token(self, mint, price):
        try:
            prices = json.loads(self.state.get(custom_token_prices))
        except TypeError: # not initalized
            prices = {}
        prices[mint] = float(price)
        self.state.set(custom_token_prices, json.dumps(prices))

    def get_tokens_data(self):
        metadata = self.get_token_metadata()
        prices = self.get_token_prices()
        data = {}
        for mint, price in prices.items():
            for token in metadata:
                if token.get('address') == mint:
                    token['price_usd'] = price
                    data[mint] = token
        return data

    def get_tokens_metadata(self):
        metadata = self.get_token_metadata()
        prices = self.get_token_prices()
        data = {}
        for token in metadata:
            mint = token.get('address')
            price = prices.get(mint)
            if price is None:
                price = 0
            token['price_usd'] = price
            data[mint] = token
        return data

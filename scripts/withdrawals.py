import requests
import datetime

contracts = [
    '8HKQYKo9Fr6zV3xJvaUX6QEfPTbyhARDzzHDZvrqbzoP',
    '5WqxtKrzFvNQUghsWycf6hsFzbwTnmJeBWMHkRbpMjcM',
    'WCSeDEJH6EYEmvLEjQisgcVAfbjJiCpQT5f2tXf9CSz',
    '3BifjKGrXe69vomZz6nRjw6mL49rv4Vk659DGfka8rsY',
    '878ZkXDutZ2mG7jMJ2ysTGE7KF1BXnaaHbfBTLC9xxnK',
    '9GSc5JWLvn57yrQes6Bvig2eXf9XLruQogKFerMiToyr',
    '5nQ9LT4TB5JS7xrw1f4rSry64dMCRCLczGibPtUZrFFD',
    '2Kz4xJP7tWnRVQnVp5McD54xtJZ5jDw157Sde89KdHa1',
    '34ZanTui6AE46UJ8jFN1nNmLYC2dCWKnPT2zesC6keP8',
    'H65p6aHfbwTAsRnWibzi9rd9P4tggHpQBBGb8wEYVo5t',
    '63PvMqG82rWyRADCe2iW1435Q4H4vNJTWJY3FTz'
]

resp = requests.get('https://maximus.streamflow.finance/api/contracts/streamflow').json()
for contract in resp:
    addre = contract.get('contract_address')
    if addre not in contracts:
        continue
    ix = contract.get('ix')
    if not bool(ix.get('automatic_withdrawal')):
        print(f"{addre} AW not enabled")
        continue
    start = int(ix.get('start_time'))
    cliff = int(ix.get('cliff'))
    if cliff > 0:
        start = cliff
    withdraw_at = start + int(ix.get('withdraw_frequency'))
    print(ix.get('cliff'))
    print(f"{addre} {datetime.datetime.fromtimestamp(ix.get('cliff'))}")

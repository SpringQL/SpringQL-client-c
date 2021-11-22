import datetime
import random
import json


def gen_trade():
    return {
        'ts': current_timestamp(),
        'ticker': gen_ticker(),
        'amount': gen_amount(),
    }


def current_timestamp():
    now = datetime.datetime.now()
    return now.strftime("%Y-%m-%d %H:%M:%S.%f") + '000'


def gen_ticker():
    return random.choice([
        'ORCL'
    ])


def gen_amount():
    return 100


print(json.dumps(gen_trade()))

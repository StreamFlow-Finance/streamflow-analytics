from flask import Flask
import os
from handlers.tokens import tokens_handler
from handlers.contracts import contracts_handler

application = Flask(__name__)
application.config['JSON_SORT_KEYS'] = False
application.config['AUTH_TOKEN'] = os.environ.get('AUTH_TOKEN')

application.register_blueprint(contracts_handler, url_prefix='/contracts')
application.register_blueprint(tokens_handler, url_prefix='/tokens')

if __name__ == "__main__":
    port = int(os.environ.get("PORT", 5000))
    application.run(host='0.0.0.0', port=port, debug=True)

from flask import Flask, request

app = Flask(__name__)

@app.route('/log-failure', methods=['POST'])
def log_failure():
    """
    Receives stale entity data and prints it to the console.
    """
    if request.is_json:
        data = request.get_json()
        print(f"--- MOCK API RECEIVED STALE ENTITIES ---")
        print(data)
        print(f"----------------------------------------")
        return "Logged", 200
    return "Bad Request: JSON expected", 400

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)



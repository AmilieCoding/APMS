from flask import Flask, send_file, jsonify
import os

app = Flask(__name__)

@app.route("/packages/neofetch.json")
def get_neofetch_info():
    return jsonify({
        "name": "neofetch",
        "version": "7.1.0",
        "download_url": "http://localhost:8080/downloads/neofetch-7.1.0.tar.gz"
    })

@app.route("/downloads/neofetch-7.1.0.tar.gz")
def download_neofetch():
    return send_file(
        "build/neofetch-7.1.0.tar.gz",
        as_attachment=True
    )

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)

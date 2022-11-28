from crypt import methods
from flask import Flask, render_template , url_for, redirect, request
import os
import psycopg2
from rsa import PublicKey
import hashlib, secrets


from Crypto.PublicKey import RSA,ECC
from Crypto.Hash import SHA256
from Crypto.Signature import PKCS1_v1_5
import hashlib, secrets

from Crypto.PublicKey import ECC

from ellipticcurve.ecdsa import Ecdsa
from ellipticcurve.privateKey import PrivateKey

def sign_request(identifier, validation_url):
    key = ECC.generate(curve='P-256')

    public_key = key.public_key().export_key(format='SEC1')
    private_key = key.export_key()

    identifier = bytes(identifier,'UTF-8')
    validation_url = bytes(validation_url,'UTF-8')

    message = identifier + validation_url
    digest = SHA256.new()
    digest.update(message)

    # Sign the message
    signer = PKCS1_v1_5.new(private_key)
    sig = signer.sign(digest)


    final_signature = sig.hex()
    final_pub_key = public_key.hex()

    return final_signature, final_pub_key


app = Flask(__name__)

def get_db_connection():
    conn = psycopg2.connect(host = 'localhost',
                            database='flask_db',
                            user=os.environ['DB_USERNAME'],
                            password=os.environ['DB_PASSWORD'])
    return conn



@app.route('/')
def index():
    conn = get_db_connection()
    cur = conn.cursor()
    cur.execute('SELECT * FROM scData;')
    scData = cur.fetchall()
    cur.close()
    conn.close()
    return render_template('index.html', scData=scData)

@app.route('/pages/identity_create')
def create_identity_page():
    return render_template('identity.html')

@app.route('/identity/create', methods=['POST'])
def identity_create():
    identity = request.form['identity']
    validation_url = request.form['validationUrl']
    signature, public_key = sign_request(identity, validation_url)


    return render_template('identity_create_success.html', validation_url=validation_url, public_key=public_key, signature=signature)

if __name__ == '__main__':
    app.run(debug=True)

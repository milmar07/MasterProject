"""@app.route('/pages/identity_create')
def create_identity_page():
    return render_template('identity.html')

@app.route('/identity/create', methods=['POST'])
def identity_create():
    identity = request.form['identity']
    validation_url = request.form['validationUrl']
    signature, public_key = sign_request(identity, validation_url)


    return render_template('identity_create_success.html', validation_url=validation_url, public_key=public_key, signature=signature)
"""
"""def sign_request(identifier, validation_url):
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
"""

from flask import Flask, render_template, url_for, redirect, request, jsonify
import os
import time
import psycopg2
from rsa import PublicKey
import hashlib, secrets
from flask_sqlalchemy import SQLAlchemy
from models import db, Organization, Sensor, IdentityClaim, SensorData, Declaration
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.hazmat.backends import default_backend
import base64


app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgresql://sammy:password@localhost/flask_db'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False

db.init_app(app)


def sign_data(private_key_pem, data):
    private_key = serialization.load_pem_private_key(
        private_key_pem.encode(),
        password=None,
        backend=default_backend()
    )

    signature = private_key.sign(
        data.encode(),
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )

    return base64.b64encode(signature).decode('utf-8')


def generate_rsa_key_pair():
    private_key = rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048,
        backend=default_backend()
    )
    public_key = private_key.public_key()

    # Serialize the private key to PEM format
    pem_private_key = private_key.private_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.NoEncryption()
    )

    # Serialize the public key to PEM format
    pem_public_key = public_key.public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo
    )

    return pem_private_key.decode('utf-8'), pem_public_key.decode('utf-8')



@app.route('/ttn', methods=['POST'])
def ttn_data():
    data = request.get_json()
    print("Received data from TTN:", data)

    device_id = data['end_device_ids']['device_id']
    temperature = data['uplink_message']['decoded_payload']['temperature']
    print("Device ID:", device_id)
    print("Temperature:", temperature)

    sensor = Sensor.query.filter_by(id=device_id).first()
    if sensor:
        organization = Organization.query.filter_by(id=sensor.organization_id).first()
        if organization:
            new_sensor_data = SensorData(sensor_id=sensor.id, temperature_reading=temperature, timestamp=int(time.time()))
            db.session.add(new_sensor_data)
            db.session.commit()
            print(f"Data saved for organization {organization.name}")
        else:
            print("Organization not found")
    else:
        print("Sensor not found")

    return jsonify({"status": "success"})


@app.route('/create_organization', methods=['GET', 'POST'])
def create_organization():
    if request.method == 'POST':
        # Extract form data
        name = request.form['name']
        contact_details = request.form['contact_details']
        public_key = request.form['public_key']

        # Create a new organization in the database
        new_organization = Organization(name=name, contact_details=contact_details, public_key=public_key)
        db.session.add(new_organization)
        db.session.commit()

        return redirect(url_for('index'))

    return render_template('create_organization.html')


@app.route('/create_sensor', methods=['GET', 'POST'])
def create_sensor():
    if request.method == 'POST':
        device_eui = request.form['id']
        organization_id = request.form['organization_id']
        sensor_type = request.form['sensor_type']
        location = request.form['location']
        additional_information = request.form['additional_information']

        new_sensor = Sensor(id=device_eui,
                            organization_id=organization_id,
                            sensor_type=sensor_type,
                            location=location,
                            additional_information=additional_information)

        db.session.add(new_sensor)
        db.session.commit()

        return redirect(url_for('index'))
    return render_template('create_sensor.html')


"""@app.route('/create_identity_claim', methods=['GET', 'POST'])
def create_identity_claim():
    if request.method == 'POST':
        # Extract form data
        sensor_id = request.form['sensor_id']
        organization_id = request.form['organization_id']
        validation_url = request.form['validation_url']
        public_key = request.form['public_key']
        signature = request.form['signature']

        # Create a new identity claim in the database
        new_identity_claim = IdentityClaim(sensor_id=sensor_id, organization_id=organization_id, validation_url=validation_url, public_key=public_key, signature=signature)
        db.session.add(new_identity_claim)
        db.session.commit()

        return redirect(url_for('index'))

    return render_template('create_identity_claim.html')"""

"""@app.route('/create_identity_claim', methods=('GET', 'POST'))
def create_identity_claim():
    if request.method == 'POST':
        sensor_id = request.form['sensor_id']
        organization_id = request.form['organization_id']
        validation_url = request.form['validation_url']
        public_key = request.form['public_key']
        private_key_pem = request.form['private_key']

        # The data you want to sign (you can modify this according to your needs)
        data_to_sign = f"{sensor_id}-{organization_id}-{validation_url}-{public_key}"

        # Generate the signature
        signature = sign_data(private_key_pem, data_to_sign)

        identity_claim = IdentityClaim(sensor_id=sensor_id, organization_id=organization_id, validation_url=validation_url, public_key=public_key, signature=signature)
        db.session.add(identity_claim)
        db.session.commit()

        return redirect(url_for('index'))

    sensors = Sensor.query.all()
    organizations = Organization.query.all()
    return render_template('create_identity_claim.html', sensors=sensors, organizations=organizations)"""

@app.route('/create_identity_claim', methods=('GET', 'POST'))
def create_identity_claim():
    if request.method == 'POST':
        sensor_id = request.form['sensor_id']
        organization_id = request.form['organization_id']
        validation_url = request.form['validation_url']

        # Generate the RSA key pair
        private_key_pem, public_key = generate_rsa_key_pair()

        # The data you want to sign (you can modify this according to your needs)
        data_to_sign = f"{sensor_id}-{organization_id}-{validation_url}-{public_key}"

        # Generate the signature
        signature = sign_data(private_key_pem, data_to_sign)

        identity_claim = IdentityClaim(sensor_id=sensor_id, organization_id=organization_id, validation_url=validation_url, public_key=public_key, signature=signature)
        db.session.add(identity_claim)
        db.session.commit()

        return redirect(url_for('index'))

    sensors = Sensor.query.all()
    organizations = Organization.query.all()
    return render_template('create_identity_claim.html', sensors=sensors, organizations=organizations)

@app.route('/create_declaration', methods=['GET', 'POST'])
def create_declaration():
    if request.method == 'POST':
        # Extract form data
        sensor_id = request.form['sensor_id']
        organization_id = request.form['organization_id']
        declaration_type = request.form['declaration_type']
        declaration_details = request.form['declaration_details']
        timestamp = request.form['timestamp']
        signature = request.form['signature']

        # Create a new declaration in the database
        new_declaration = Declaration(sensor_id=sensor_id, organization_id=organization_id, declaration_type=declaration_type, declaration_details=declaration_details, timestamp=timestamp, signature=signature)
        db.session.add(new_declaration)
        db.session.commit()

        return redirect(url_for('index'))

    return render_template('create_declaration.html')

from flask import jsonify, request

@app.route('/validation', methods=['GET'])
def validate_identity_claim():
    # Extract the identity claim information from the request (e.g., sensor ID, organization ID, signature)
    # This could be sent as query parameters or in the request headers.
    sensor_id = request.args.get('sensor_id')
    organization_id = request.args.get('organization_id')
    signature = request.args.get('signature')

    # Perform the validation process:
    # 1. Retrieve the identity claim associated with the given sensor_id and organization_id from the database
    # 2. Verify the provided signature using the organization's public key
    # 3. Return a response indicating the validation result (e.g., success or failure)

    # ... (perform validation here) ...

    # Assuming the validation process is successful
    response = {
        'status': 'success',
        'message': 'Identity claim is valid.'
    }
    return jsonify(response)


def get_db_connection():
    conn = psycopg2.connect

@app.route('/')
def index():
    organizations = Organization.query.all()
    sensors = Sensor.query.all()
    identity_claims = IdentityClaim.query.all()
    declarations = Declaration.query.all()
    sensor_data = SensorData.query.all()

    return render_template('index.html', 
                            organizations=organizations, 
                            sensors=sensors, 
                            identity_claims=identity_claims, 
                            declarations=declarations,
                            sensor_data=sensor_data)


"""@app.route('/')
def index():
    scData = SensorData.query.all()
    return render_template('index.html', scData=scData)"""


if __name__ == '__main__':
    app.run(debug=True)

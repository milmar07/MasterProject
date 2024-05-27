from flask import Flask, jsonify, render_template, url_for, redirect, request, send_file
import paho.mqtt.client as mqtt
import json
import time
import csv
from datetime import datetime
from models import db, Organization, Sensor, SensorData
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.hazmat.backends import default_backend
import base64
import requests

app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgresql://sammy:password@localhost/flask_db'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False

db.init_app(app)

# MQTT client details
mqtt_broker = "eu1.cloud.thethings.network"
mqtt_port = 1883
mqtt_username = "your_application_id@tenant"
mqtt_password = "your_api_key"
mqtt_topic = "v3/your_application_id@tenant/devices/your_device_id/up"

received_data = []

def on_connect(client, userdata, flags, rc):
    print("Connected with result code " + str(rc))
    client.subscribe(mqtt_topic)

def on_message(client, userdata, msg):
    data = json.loads(msg.payload)
    received_data.append(data)
    print(f"Received message: {data}")
    process_sensor_data(data)

def process_sensor_data(data):
    device_id = data['end_device_ids']['device_id']
    application_id = data['end_device_ids']['application_ids']['application_id']
    temperature = data['uplink_message']['decoded_payload']['temperature']
    print("Device ID:", device_id)
    print("Temperature:", temperature)

    ttn_time_str = data['received_at']
    ttn_time = datetime.strptime(ttn_time_str, "%Y-%m-%dT%H:%M:%S.%fZ")
    is_valid = False

    sensor = Sensor.query.filter_by(sensor_id=device_id).first()
    if sensor:
        organization = Organization.query.filter_by(id=sensor.org_id).first()
        if organization:
            new_sensor_data = SensorData(sensor_id=sensor.id, temperature_reading=temperature, timestamp=int(time.time()))
            db.session.add(new_sensor_data)
            db.session.commit()
            print(f"Data saved for organization {organization.name}")

            # Sign the data
            signed_data = sign_data(organization.private_key, str(temperature))

            # Call the smart contract for validation
            response = wasp_validate_sensor_data(sensor_id=device_id, signed_data=signed_data)

            # Get the current time
            end_time = time.time()

            # Calculate the time difference
            time_difference = end_time - ttn_time.timestamp()
        
            # If the smart contract validation is successful, set the is_valid flag to True
            if response.get('is_valid'):
                new_sensor_data.is_valid = True
                db.session.commit()
                is_valid = True

        else:
            print("Organization not found")
            end_time = time.time()
    else:
        print("Sensor not found")
        end_time = time.time()

    time_difference = end_time - ttn_time.timestamp()

    # Write data to CSV file
    with open('time.csv', mode='a', newline='') as csvfile:
        csv_writer = csv.writer(csvfile)
        csv_writer.writerow([device_id, time_difference, application_id, temperature, is_valid, SMART_CONTRACT_NAME])

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

def wasp_send_request(endpoint, data=None, method="GET"):
    url = f"{get_wasp_node_url()}/chain/{CHAIN_ID}/{SMART_CONTRACT_NAME}/{endpoint}"
    if method == "POST":
        response = requests.post(url, json=data)
    else:
        response = requests.get(url, json=data)
    return response.json()

def wasp_validate_sensor_data(sensor_id, signed_data):
    data = {
        "chain_id": CHAIN_ID,
        "sensor_id": sensor_id,
        "signed_data": signed_data,
    }
    response = wasp_send_request("validateSensorData", data, method="POST")
    return response

def get_wasp_node_url():
    return "http://localhost:9090"  # Adjust the URL to your IOTA Wasp node's address and port

mqtt_client = mqtt.Client()
mqtt_client.username_pw_set(mqtt_username, mqtt_password)
mqtt_client.on_connect = on_connect
mqtt_client.on_message = on_message
mqtt_client.connect(mqtt_broker, mqtt_port, 60)
mqtt_client.loop_start()

@app.route('/data', methods=['GET'])
def get_data():
    return jsonify(received_data)

@app.route('/create_organization', methods=['GET', 'POST'])
def create_organization():
    if request.method == 'POST':
        org_id = request.json['org_id']
        name = request.json['name']

        # Generate the RSA key pair
        private_key, public_key = generate_rsa_key_pair()

        # Create a new organization with the generated public key and private key
        new_organization = Organization(org_id=org_id, name=name, public_key=public_key)

        # Call the smart contract to register the organization
        wasp_create_organization(CHAIN_ID, org_id, public_key)

        db.session.add(new_organization)
        db.session.commit()

        return jsonify({'private_key': private_key})
    return render_template('create_organization.html')

@app.route('/create_sensor', methods=['GET', 'POST'])
def create_sensor():
    if request.method == 'POST':
        sensor_id = request.form['sensor_id']
        sensor_type = request.form['sensor_type']
        location = request.form['location']
        org_id = request.form['org_id']

        new_sensor = Sensor(sensor_id=sensor_id, sensor_type=sensor_type, location=location, org_id=org_id)

        # Call the smart contract to register the sensor
        wasp_create_sensor(CHAIN_ID, sensor_id, org_id)

        db.session.add(new_sensor)
        db.session.commit()

        return redirect(url_for('index'))
    return render_template('create_sensor.html')

@app.route('/')
def index():
    organizations = Organization.query.all()
    sensors = Sensor.query.all()
    sensor_data = SensorData.query.all()

    return render_template('index.html', 
                            organizations=organizations, 
                            sensors=sensors, 
                            sensor_data=sensor_data)

if __name__ == '__main__':
    app.run(debug=True)

import paho.mqtt.client as mqtt
import json
import time
import random

# TTN MQTT broker details
mqtt_broker = "eu1.cloud.thethings.network"
mqtt_port = 1883
mqtt_username = "your_application_id@tenant"
mqtt_password = "your_api_key"
mqtt_topic = "v3/your_application_id@tenant/devices/your_device_id/up"

# Initialize the temperature reading
current_temperature = 29.64  # Starting temperature

def on_connect(client, userdata, flags, rc):
    if rc == 0:
        print("Connected successfully")
    else:
        print("Connection failed with code " + str(rc))

def on_publish(client, userdata, mid):
    print("Message published: " + str(mid))

def simulate_temperature_sensor():
    global current_temperature

    client = mqtt.Client()
    client.username_pw_set(mqtt_username, mqtt_password)
    client.on_connect = on_connect
    client.on_publish = on_publish

    try:
        client.connect(mqtt_broker, mqtt_port, 60)
        client.loop_start()

        while True:
            # Determine whether to increase or decrease the temperature
            change_direction = random.choice([-1, 1])
            # Generate a random change value between 0.01 and 0.1
            temperature_change = round(random.uniform(0.01, 0.1), 2) * change_direction
            # Apply the change to the current temperature and ensure two decimal places
            current_temperature = round(current_temperature + temperature_change, 2)
            
            # Create the payload
            payload = {
                "temperature": current_temperature,
                "timestamp": int(time.time())
            }
            payload = json.dumps(payload)
            
            # Publish the sensor data to the MQTT topic
            result = client.publish(mqtt_topic, payload)
            status = result[0]
            if status == 0:
                print(f"Sent `{payload}` to topic `{mqtt_topic}`")
            else:
                print(f"Failed to send message to topic {mqtt_topic}")
            
            # Wait for 30 seconds before the next reading
            time.sleep(30)

    except Exception as e:
        print(f"Error: {e}")
    finally:
        client.loop_stop()
        client.disconnect()

if __name__ == "__main__":
    simulate_temperature_sensor()

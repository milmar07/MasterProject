# IoT Data Validation and Storage with IOTA Wasp Smart Contracts

This project is a web application designed to receive, validate, and store data from IoT sensors. The application leverages IOTA Wasp smart contracts to ensure the integrity and authenticity of the sensor data. By utilizing blockchain technology, the solution provides a transparent and tamper-proof way to store and validate data from various IoT devices.

The key features of the application include:

1. **Receiving data from IoT sensors:** The application is capable of receiving data from IoT devices in JSON format through a dedicated endpoint.
2. **Storing data in a PostgreSQL database:** Sensor data is stored in a PostgreSQL database, making it easy to manage and query the information.
3. **Validating data using IOTA Wasp smart contracts:** The application interacts with an IOTA Wasp smart contract to validate the received sensor data. This ensures the data's authenticity and integrity, providing a higher level of trust in the information.
4. **Web interface for managing organizations and sensors:** The application features a user-friendly web interface for creating and managing organizations and their associated sensors. This allows users to easily set up and maintain their IoT infrastructure.

This solution is ideal for organizations that require a secure and reliable way to store and validate data from their IoT devices. By using IOTA Wasp smart contracts, the application provides a cutting-edge and transparent method for ensuring the trustworthiness of sensor data.


## Table of Contents

1. [Requirements](#requirements)
2. [Installation Guide for Ubuntu Linux](#installation-guide-for-ubuntu-linux)
3. [Usage](#usage)
4. [License](#license)

## Requirements

- Python 3.6+
- PostgreSQL
- Flask
- Flask-SQLAlchemy
- psycopg2
- Requests
- Cryptography
- IOTA Wasp node


## Installation Guide for Ubuntu Linux

This guide will walk you through the installation process for each component required to run the IoT Data Validation and Storage with IOTA Wasp Smart Contracts web application on Ubuntu Linux.

### 1. Install Python 3 and pip

Ubuntu 20.04 and later versions come with Python 3.8 pre-installed. To check if you have Python 3 installed, open a terminal and run:

```
python3 --version
```

If Python 3 is not installed, run the following command:

```
sudo apt-get update
sudo apt-get install python3
```

Install pip, the Python package manager:

```
sudo apt-get install python3-pip
```

### 2. Install Virtualenv

Install the virtualenv package to create isolated Python environments:

```
pip3 install virtualenv
```

### 3. Clone the Repository

Clone the project repository from GitHub:

```
git clone https://github.com/milmar07/MasterProject.git
cd WebAppPythonFlask
```

### 4. Create a Virtual Environment and Install Dependencies

Create a virtual environment and activate it:

```
virtualenv venv
source venv/bin/activate
```

Install the required Python packages using the provided `requirements.txt` file:

```
pip install -r requirements.txt
```

### 5. Install PostgreSQL

Install PostgreSQL using the following commands:

```
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib
```

### 6. Configure PostgreSQL

Switch to the PostgreSQL user account:

```
sudo -iu postgres psql
```

Create a flask_db for your project:

```
CREATE DATABASE flask_db;
```

Create a database user for your project

```
CREATE USER sammy WITH PASSWORD 'password';

Grant all privileges to the new user on the database:

```
GRANT ALL PRIVILEGES ON DATABASE flask_db TO sammy;
```

Check if database is created:

```
\l
````

Exit out the PostgreSQL prompt by typing:

```
\q
```

Exit the PostgreSQL user account:

```
exit
```

## Usage

1. Run the application:

   ```
   python app.py
   ```

2. Open a web browser and navigate to http://127.0.0.1:5000/.

3. Use the web interface to create organizations and sensors, and interact with your IOTA Wasp smart contract.

## License

Your project is licensed under the [MIT License](LICENSE).

---

# IoT Data Validation and Storage with IOTA Wasp Smart Contracts

This project is a web application designed to receive, validate, and store data from IoT sensors. The application leverages IOTA Wasp smart contracts to ensure the integrity and authenticity of the sensor data. By utilizing blockchain technology, the solution provides a transparent and tamper-proof way to store and validate data from various IoT devices.

The key features of the application include:

Receiving data from IoT sensors: The application is capable of receiving data from IoT devices in JSON format through a dedicated endpoint.
Storing data in a PostgreSQL database: Sensor data is stored in a PostgreSQL database, making it easy to manage and query the information.
Validating data using IOTA Wasp smart contracts: The application interacts with an IOTA Wasp smart contract to validate the received sensor data. This ensures the data's authenticity and integrity, providing a higher level of trust in the information.
Web interface for managing organizations and sensors: The application features a user-friendly web interface for creating and managing organizations and their associated sensors. This allows users to easily set up and maintain their IoT infrastructure.
This solution is ideal for organizations that require a secure and reliable way to store and validate data from their IoT devices. By using IOTA Wasp smart contracts, the application provides a cutting-edge and transparent method for ensuring the trustworthiness of sensor data.
## Table of Contents

1. [Requirements](#requirements)
2. [Installation](#installation)
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

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/your-repo-name.git
   cd your-repo-name
   ```

2. Create a virtual environment and activate it:

   ```
   python3 -m venv venv
   source venv/bin/activate  # On Windows: .\venv\Scripts\activate
   ```

3. Install the required packages:

   ```
   pip install -r requirements.txt
   ```

4. Set up the environment variables for your database:

   ```
   export DATABASE_URL="postgresql://username:password@localhost/db_name"  # On Windows: set DATABASE_URL=postgresql://username:password@localhost/db_name
   ```

5. Create the necessary tables in your PostgreSQL database:

   ```
   python create_tables.py
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

Please customize the Readme.md file to better reflect the specific details of your project. Replace placeholders like "Your Project Name", "yourusername", and "your-repo-name" with the appropriate information. Also, make sure to add any additional information or sections that you think would be useful for users of your project.

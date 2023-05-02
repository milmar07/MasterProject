# Secure IoT Data Management System using IOTA and Smart Contracts

This project aims to implement a secure IoT data management system using IOTA distributed ledger technology and Wasp Smart Contracts. The primary goal is to create a transparent, tamper-resistant, and efficient system for data validation and storage, improving security and trust within the IoT ecosystem.

## Technologies and Components
- IOTA: A distributed ledger technology specifically designed for the Internet of Things, providing a scalable and feeless solution for secure data and value transfer.
- IOTA Wasp Smart Contracts: A layer built on top of the IOTA network that enables the creation and execution of smart contracts, offering advantages such as decentralization, automation, scalability, and tamper-resistance.
- Flask: A lightweight, flexible, and easy-to-use Python-based web framework used for developing web applications.
- The Things Network (TTN): A global, open-source IoT data network that serves as a data communication hub, forwarding IoT data to our web application.

## Project Overview
The overall architecture of the project consists of four main components: IoT devices, The Things Network (TTN), the web application, and the IOTA Wasp Smart Contracts platform.

1. **IoT Devices**: IoT devices are responsible for collecting data, such as temperature, and sending it to The Things Network.
2. **The Things Network (TTN)**: TTN serves as a data communication hub, forwarding the IoT data to our web application.
3. **Web Application**: Built using Flask, the web application securely manages and stores the IoT data, and interacts with the IOTA Wasp Smart Contracts platform for data validation.
4. **IOTA Wasp Smart Contracts**: The platform provides the underlying blockchain-based infrastructure that ensures the security, integrity, and validation of the IoT data.

## Project Features
- **Web Application**: Allows users to create organizations, register sensors, and display data. Implements RSA key pair generation for signing and validating data.
- **Smart Contract**: Serves as the core of the data validation process, securely storing public keys of registered organizations and providing methods for creating new organizations and sensors, as well as validating sensor data.
- **Interaction between Components**: The web application communicates with the smart contract via the IOTA Wasp platform's API. IoT devices send data to the web application, which signs the data using the organization's private key before sending it to the smart contract for validation.

## Getting Started
Please refer to the individual directories within this repository for more detailed information on setting up and using each component of the project.

1. [Web Application](./WebAppPythonFlask/README.md): Setup and usage instructions for the Flask web application.
2. [IOTA Wasp Smart Contracts](./wasp-0.3.8/README.md): Information on deploying and interacting with the smart contract on the IOTA Wasp platform.

## License
This project is licensed under the [MIT License](./LICENSE).

import os
import psycopg2

def drop_tables(conn):
    cursor = conn.cursor()
        
    cursor.execute("DROP TABLE IF EXISTS sensor_readings CASCADE")
    cursor.execute("DROP TABLE IF EXISTS sensors CASCADE")
    cursor.execute("DROP TABLE IF EXISTS organizations CASCADE")


def create_tables(conn):
    cursor = conn.cursor()
    
    cursor.execute("""
    CREATE TABLE organizations (
        id SERIAL PRIMARY KEY,
        org_id VARCHAR(80) UNIQUE NOT NULL,
        name VARCHAR(255) UNIQUE,
        public_key VARCHAR(2048) NOT NULL,
        private_key VARCHAR(2048) NOT NULL
    );
    """)

    cursor.execute("""
    CREATE TABLE sensors (
        id SERIAL PRIMARY KEY,
        sensor_id VARCHAR(255) UNIQUE,
        sensor_type VARCHAR(255),
        location VARCHAR(255),
        org_id INTEGER REFERENCES organizations(id)
    );
    """)

    cursor.execute("""
    CREATE TABLE sensor_readings (
        id SERIAL PRIMARY KEY,
        sensor_id INTEGER REFERENCES sensors(id),
        timestamp TIMESTAMP,
        temperature_reading FLOAT,
        is_valid BOOLEAN
    );
    """)

    conn.commit()

def main():
    conn = psycopg2.connect(
        host="localhost",
        database="flask_db",
        user='sammy',
        password='password')

    #Drop tables
    drop_tables(conn)
    #Create tables
    create_tables(conn)

    #Close connection
    conn.close()

if __name__ == "__main__":
    main()
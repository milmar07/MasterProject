import os
import psycopg2


def drop_tables(conn):
    cursor = conn.cursor()
    
    cursor.execute("DROP TABLE IF EXISTS declarations")
    cursor.execute("DROP TABLE IF EXISTS sensor_data")
    cursor.execute("DROP TABLE IF EXISTS identity_claims")
    cursor.execute("DROP TABLE IF EXISTS sensors")
    cursor.execute("DROP TABLE IF EXISTS organizations")

def create_tables(conn):
    cursor = conn.cursor()
    
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS organizations (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        contact_details TEXT,
        public_key TEXT NOT NULL
    )
    """)

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS sensors (
        id TEXT PRIMARY KEY,
        organization_id INTEGER,
        sensor_type TEXT NOT NULL,
        location TEXT NOT NULL,
        additional_information TEXT,
        FOREIGN KEY (organization_id) REFERENCES organizations (id)
    )
    """)

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS identity_claims (
        id SERIAL PRIMARY KEY,
        sensor_id TEXT,
        organization_id INTEGER,
        validation_url TEXT NOT NULL,
        public_key TEXT NOT NULL,
        signature TEXT NOT NULL,
        FOREIGN KEY (sensor_id) REFERENCES sensors (id),
        FOREIGN KEY (organization_id) REFERENCES organizations (id)
    )
    """)

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS sensor_data (
        id SERIAL PRIMARY KEY,
        sensor_id TEXT,
        timestamp INTEGER NOT NULL,
        temperature_reading REAL NOT NULL,
        signed_data TEXT NOT NULL,
        FOREIGN KEY (sensor_id) REFERENCES sensors (id)
    )
    """)


    cursor.execute("""
    CREATE TABLE IF NOT EXISTS declarations (
        id SERIAL PRIMARY KEY,
        sensor_id TEXT,
        organization_id INTEGER,
        declaration_type TEXT NOT NULL,
        declaration_details TEXT,
        timestamp INTEGER NOT NULL,
        signature TEXT NOT NULL,
        FOREIGN KEY (sensor_id) REFERENCES sensors (id),
        FOREIGN KEY (organization_id) REFERENCES organizations (id)
    )
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


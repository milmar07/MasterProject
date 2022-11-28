import os
import psycopg2

conn = psycopg2.connect(
    host="localhost",
    database="flask_db",
    user=os.environ['DB_USERNAME'],
    password=os.environ['DB_PASSWORD'])

#Open a cursor to preform database operations
cur = conn.cursor()

#Execute a command: this creates a new table
cur.execute('DROP TABLE IF EXISTS scData')
cur.execute('DROP TABLE IF EXISTS identityClaimData')
cur.execute('DROP TABLE IF EXISTS declarationData')
cur.execute('CREATE TABLE identityClaimData ( id serial PRIMARY KEY, identity varchar (150) NOT NULL, signature varchar (150) NOT NULL, publicKey varchar (150) NOT NULL, validationUrl varchar(150) NOT NULL);')

#Insert data into the table
#Here we need to add some function in order to get the data  
cur.execute('INSERT INTO scData (identity, signature, publicKey, validationUrl) VALUES (%s, %s, %s, %s)', ("Identity1", "MarkoMarkovic", "PubKey1", "https://localhost:1234"))


conn.commit()


cur.close()
conn.close()


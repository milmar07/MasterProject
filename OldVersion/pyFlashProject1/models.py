from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class Organization(db.Model):
    __tablename__ = 'organizations'
    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    name = db.Column(db.String, nullable=False)
    contact_details = db.Column(db.String)
    public_key = db.Column(db.String, nullable=False)

class Sensor(db.Model):
    __tablename__ = 'sensors'
    id = db.Column(db.String, primary_key=True)
    organization_id = db.Column(db.Integer, db.ForeignKey('organizations.id'))
    sensor_type = db.Column(db.String, nullable=False)
    location = db.Column(db.String, nullable=False)
    additional_information = db.Column(db.String)

    organization = db.relationship('Organization', backref=db.backref('sensors', lazy=True))

class IdentityClaim(db.Model):
    __tablename__ = 'identity_claims'
    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    sensor_id = db.Column(db.String, db.ForeignKey('sensors.id'))
    organization_id = db.Column(db.Integer, db.ForeignKey('organizations.id'))
    validation_url = db.Column(db.String, nullable=False)
    public_key = db.Column(db.String, nullable=False)
    signature = db.Column(db.String, nullable=False)

    sensor = db.relationship('Sensor', backref=db.backref('identity_claims', lazy=True))
    organization = db.relationship('Organization', backref=db.backref('identity_claims', lazy=True))

class SensorData(db.Model):
    __tablename__ = 'sensor_data'
    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    sensor_id = db.Column(db.String, db.ForeignKey('sensors.id'))
    timestamp = db.Column(db.Integer, nullable=False)
    temperature_reading = db.Column(db.Float, nullable=False)
    signed_data = db.Column(db.String, nullable=False)

    sensor = db.relationship('Sensor', backref=db.backref('sensor_data', lazy=True))

class Declaration(db.Model):
    __tablename__ = 'declarations'
    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    sensor_id = db.Column(db.String, db.ForeignKey('sensors.id'))
    organization_id = db.Column(db.Integer, db.ForeignKey('organizations.id'))
    declaration_type = db.Column(db.String, nullable=False)
    declaration_details = db.Column(db.String)
    timestamp = db.Column(db.Integer, nullable=False)
    signature = db.Column(db.String, nullable=False)

    sensor = db.relationship('Sensor', backref=db.backref('declarations', lazy=True))
    organization = db.relationship('Organization', backref=db.backref('declarations', lazy=True))

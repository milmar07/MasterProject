from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class Organization(db.Model):
    __tablename__ = 'organizations' 
    id = db.Column(db.Integer, primary_key=True)
    org_id = db.Column(db.String(80), unique=True, nullable=False)
    name = db.Column(db.String(255), unique=True)
    public_key = db.Column(db.String(2048), nullable=False)

    def __repr__(self):
        return f"<Organization {self.name}>"



class Sensor(db.Model):
    __tablename__ = 'sensors'

    id = db.Column(db.Integer, primary_key=True)
    sensor_id = db.Column(db.String(255), unique=True)
    sensor_type = db.Column(db.String(255))
    location = db.Column(db.String(255))
    org_id = db.Column(db.Integer, db.ForeignKey('organizations.id'))

    sensor_readings = db.relationship('SensorData', backref='sensor', lazy=True)

class SensorData(db.Model):
    __tablename__ = 'sensor_readings'

    id = db.Column(db.Integer, primary_key=True)
    sensor_id = db.Column(db.Integer, db.ForeignKey('sensors.id'))
    timestamp = db.Column(db.DateTime)
    temperature_reading = db.Column(db.Float)
    is_valid = db.Column(db.Boolean)


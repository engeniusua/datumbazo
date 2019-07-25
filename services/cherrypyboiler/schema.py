from mongoengine import Document, StringField, DateTimeField
import datetime


class Telemetric(Document):
    sensorId = StringField(required=True)
    value = StringField(required=True)
    date = DateTimeField(default=datetime.datetime.utcnow)

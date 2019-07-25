import cherrypy
import os
from schema import Telemetric


class TelemetricREST(object):
    def __init__(self):
        pass

    exposed = True

    @cherrypy.tools.json_out()
    def GET(self, size=100, sensorId=None):

        if size is None:
            data = Telemetric.objects(sensorId=sensorId)
        else:
            try:
                data = Telemetric.objects[:int(size)](sensorId=sensorId)
            except Exception as e:
                raise cherrypy.HTTPError(400, str(e))

        return {
            "sensorId": sensorId,
            "data": list(map(lambda d: {
                "value": str(d.value),
                "data": str(d.date)
            }, data))
        }

    @cherrypy.tools.json_in()
    @cherrypy.tools.json_out()
    def POST(self, **params):

        m = Telemetric()
        m.sensorId = cherrypy.request.json.get('sensorId')
        m.value = cherrypy.request.json.get('value')

        try:
            m.save()
        except Exception as e:
            raise cherrypy.HTTPError(400, str(e))

        cherrypy.response.status = 200
        return {
            "response": "Posted"
        }

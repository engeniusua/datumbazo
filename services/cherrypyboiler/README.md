# Moisture REST Service

This service provides an __API__ to the moisture database through __REST__ services

## Endpoints

### `/$telemetric`

`$telemetric` is the measurement telemetric that you will be handling. This variable must be replaced with the telemetric type like `temperature` e.g.



#### GET
```bash
curl -X GET 'http://ip:port/$telemetric?sensorId=$sensorId&size=$numberofmeasurements'
```

__Request__:

Parameters:
- `sensorID` : sensor identification
- `size`: number of measurements to retrieve 

__Response__:

```json
{
    "sensorId": "3511895058",
    "data": [
        {
            "value": "1.24",
            "data": "2019-07-25T15:39:49.258+00:00"
        },
        {
            "value": "1.26",
            "data": "2019-07-25T15:39:52.518+00:00"
        },
        {
            "value": "1.22",
            "data": "2019-07-25T15:39:53.390+00:00"
        },
        ...
    ]
}
```

#### POST

```bash
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"sensor":"3511895058","value":"1.26"}' \
  http://ip:port/$telemetric
```

__Request__:

Body:

```json
{
    "sensor": "3511895058",
    "value": 1.26
}
```



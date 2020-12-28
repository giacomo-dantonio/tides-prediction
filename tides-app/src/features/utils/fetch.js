import fetchJson from "fetch-json";

const BASE_URL = "https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations";

function getUnixTimestamp(date) {
    return date.getTime() / 1000;
}

export async function fetchMeasurements(stationId) {
    const url = `${BASE_URL}/${stationId}/W/measurements?start=P30D`;
    const raw_data = await fetchJson.get(url);

    const processed_data = [];
    for (const entry of raw_data) {
        // entry.timestamp is here an rfc3xxx date.
        const date = new Date(entry.timestamp);

        processed_data.push(Object.assign(
            {},
            entry,
            {
                timestamp: getUnixTimestamp(date)
            }
        ));
    }
    return processed_data;
}

export async function fetchStations(location) {
    const url = (location !== null && location !== undefined)
        ? `${BASE_URL}?latitude=${location.latitude}&longitude=${location.longitude}&radius=${location.radius}`
        : BASE_URL;
    const data = await fetchJson.get(url);
    data.sort((lhs, rhs) => lhs.longname < rhs.longname ? -1 : 1);
    return data;
}

/* global BigInt */

import fetchJson from "fetch-json";

const BASE_URL = "https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations";

function getUnixTimestamp(date) {
    return BigInt(date.getTime() / 1000);
}

export default async function fetchMeasurements(stationId) {
    const url = `${BASE_URL}/${stationId}/W/measurements?start=P30D`;
    const raw_data = await fetchJson.get(url);

    const processed_data = [];
    for (const entry of raw_data) {
        // entry.timestamp is here an rfc3xxx date.
        const date = new Date(entry.timestamp);

        // keep only the full hours.
        if (date.getMinutes() === 0)
        {
            processed_data.push(Object.assign(
                {},
                entry,
                {
                    timestamp: getUnixTimestamp(date)
                }
            ));
        }
    }
    return processed_data;
}
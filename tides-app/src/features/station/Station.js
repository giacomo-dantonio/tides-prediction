import React from 'react';
import fetchMeasurements from './fetch';
import {Series} from 'tides-signals';
import { series_evaluate } from 'tides-signals/tides_signals_bg.wasm';

export default function Station(props) {
    const {stationId} = props;

    const measurements = fetchMeasurements(stationId);
    let series = Series.from_json(JSON.stringify(measurements));
    for (const entry of measurements) {
        entry.prediction = series.evaluate(entry.timestamp);
    }

    return <div>{measurements}</div>;
}
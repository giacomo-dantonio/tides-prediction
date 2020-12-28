/* global BigInt */

import React from 'react';
import {useSelector, useDispatch} from 'react-redux';

import {Series} from 'tides-signals';

import Chart from '../chart/Chart';
import {
    fetchMeasurements,
    selectMeasurements,
    FETCH_STATE
} from './measurementsSlice';
import {
    batchSet,
    selectPredictions
} from './predictionsSlice';
import {selectStation} from './selectedStationSlice';

const GAUGE_COLOR = "#fcc653";
const PREDICTION_COLOR = "#53c1fc";

function makeDataset(name, entries, color) {
    return {
        name,
        color,
        values: entries
    };
}

function filterHours(dataSeries) {
    return dataSeries.filter(entry => {
        const date = new Date(entry.timestamp * 1000);
        return date.getMinutes() === 0;
    });
}

export default function Station() {
    const dispatch = useDispatch();

    // FIXME show animations for fetching and computing states
    const {fetching, value: measurements} = useSelector(selectMeasurements);
    const predictions = useSelector(selectPredictions);
    const stationId = useSelector(selectStation);

    if (fetching === FETCH_STATE.INITIAL) {
        dispatch(fetchMeasurements(stationId));
        return <div/>;
    }

    const hourlyMeasurements = filterHours(measurements); 
    const dataSeries = [
        makeDataset("Measured gauge", hourlyMeasurements, GAUGE_COLOR)];

    if (fetching === FETCH_STATE.FETCHED) {
        // FIXME: find out a better way to check if predictions have
        // already be computed
        if (predictions.length === 0) {
            let series = Series.from_data(
                measurements.map(mes => BigInt(mes.timestamp)),
                measurements.map(mes => mes.value),
            );

            const timestamps = hourlyMeasurements
                .slice(Math.max(0, hourlyMeasurements.length - 10))
                .map(mes => mes.timestamp);
            const lastTimestemp = timestamps[timestamps.length - 1];
            const weekHours = 7 /* days */ * 24 /*hours*/;
            for (let i = 1; i < weekHours; i++) {
                timestamps.push(lastTimestemp + i * 3600 /* seconds */);
            }
            const values = series.batch_evaluate(
                timestamps.map(timestamp => BigInt(timestamp)));
            const computed = timestamps.map((timestamp, i) => {
                return {
                    timestamp,
                    value: values[i]
                }
            });
            dispatch(batchSet(computed));
        }
    }

    dataSeries.push(makeDataset("Prediction", predictions, PREDICTION_COLOR));
    return <div>
        <Chart series={dataSeries} />
    </div>;
}
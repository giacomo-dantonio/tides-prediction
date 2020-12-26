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

function makeDataset(name, entries, color) {
    return {
        name,
        color,
        values: entries
    };
}

export default function Station(props) {
    const {stationId} = props;
    const dispatch = useDispatch();

    // FIXME show anymations for fetching and computing states
    const {fetching, value: measurements} = useSelector(selectMeasurements);
    const predictions = useSelector(selectPredictions);

    if (fetching === FETCH_STATE.INITIAL) {
        dispatch(fetchMeasurements(stationId));
        return <div/>;
    }

    const dataSeries = [makeDataset("Measured Pegel", measurements, "red")];

    if (fetching === FETCH_STATE.FETCHED) {
        // FIXME: find out a better way to check if predictions have
        // already be computed
        if (predictions.length === 0) {
            let series = Series.from_data(
                measurements.map(mes => BigInt(mes.timestamp)),
                measurements.map(mes => mes.value),
            );
    
            const values = series.batch_evaluate(
                measurements.map(mes => BigInt(mes.timestamp)));
            const computed = measurements.map((mes, i) => {
                return {
                    timestamp: mes.timestamp,
                    value: values[i]
                }
            });
            dispatch(batchSet(computed));
        }
        else {
            dataSeries.push(makeDataset("Prediction", predictions, "blue"));
        }
    }

    return <div>
        <Chart series={dataSeries} />
    </div>;
}
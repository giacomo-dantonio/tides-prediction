/* global BigInt */

import React, {useEffect, useState} from 'react';
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
import {select as selectExtremes, set as setExtremes} from './extremesSlice';

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
    const {center} = useSelector(selectExtremes);

    const [series, setSeries] = useState(null);
    const [dataSeries, setDataSeries] = useState([]);

    useEffect(() => {
        const hourlyMeasurements = filterHours(measurements); 
        const result = [
            makeDataset("Measured gauge", hourlyMeasurements, GAUGE_COLOR)];
    
        result.push(makeDataset("Prediction", predictions, PREDICTION_COLOR));
        setDataSeries(result);
    }, [measurements, predictions])

    useEffect(computeLowHighTides, [center, series]);

    useEffect(() => {
        if (fetching === FETCH_STATE.FETCHED) {
            // FIXME: find out a better way to check if predictions have
            // already be computed
            if (predictions.length === 0) {
                const data_series = Series.from_data(
                    measurements.map(mes => BigInt(mes.timestamp)),
                    measurements.map(mes => mes.value),
                );
                setSeries(data_series);
    
                // Compute predictions
                const hourlyMeasurements = filterHours(measurements); 
                const timestamps = hourlyMeasurements
                    .slice(Math.max(0, hourlyMeasurements.length - 10))
                    .map(mes => mes.timestamp);
                const lastTimestemp = timestamps[timestamps.length - 1];
                const weekHours = 7 /* days */ * 24 /*hours*/;
                for (let i = 1; i < weekHours; i++) {
                    timestamps.push(lastTimestemp + i * 3600 /* seconds */);
                }
    
                const values = data_series.batch_evaluate(
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
    }, [fetching]);

    if (fetching === FETCH_STATE.INITIAL) {
        dispatch(fetchMeasurements(stationId));
        return <div/>;
    }

    return <div>
        <Chart series={dataSeries} />
    </div>;

    function computeLowHighTides() {
        if (series !== null) {
            // Compute low (minima) and high (maxima) tides
            const rangeWidth = 24 * 60 * 60;
            const [from, to] = [BigInt(center - rangeWidth), BigInt(center + rangeWidth)];

            const min_ts = series.find_minimum(from, to);
            const min_values = series.batch_evaluate(min_ts);
            const minima = Array.from(min_ts, (timestamp, index) => ({
                timestamp: Number(timestamp),
                value: Number(min_values[index])
            }));

            const max_ts = series.find_maximum(from, to);
            const max_values = series.batch_evaluate(max_ts);
            const maxima = Array.from(max_ts, (timestamp, index) => ({
                timestamp: Number(timestamp),
                value: Number(max_values[index])
            }));

            dispatch(setExtremes(
                {
                    center,
                    minima,
                    maxima
                }
            ));
        }
    }
}
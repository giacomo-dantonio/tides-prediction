import React, { useState } from 'react';
import { useSelector, useDispatch } from 'react-redux';

import {Series} from 'tides-signals';

import {fetchMeasurements, selectMeasurements} from './stationSlice';

export default function Station(props) {
    const {stationId} = props;
    const dispatch = useDispatch();

    const measurements = useSelector(selectMeasurements);
    let predictions = null;
    // FIXME: use state for this
    if (measurements.length === 0) {
        dispatch(fetchMeasurements(stationId));
    }
    else {
        let series = Series.from_data(
            measurements.map(mes => mes.timestamp),
            measurements.map(mes => mes.value),
        );

        predictions = measurements
            .map(entry => series.evaluate(entry.timestamp));
    }

    return <div>{measurements}</div>;
}
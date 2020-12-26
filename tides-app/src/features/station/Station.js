import React from 'react';
import { useSelector, useDispatch } from 'react-redux';

import {Series} from 'tides-signals';

import {fetchMeasurements, selectMeasurements} from './stationSlice';

export default function Station(props) {
    const {stationId} = props;
    const dispatch = useDispatch();

    const measurements = useSelector(selectMeasurements);
    // FIXME: use state for this
    if (measurements.length === 0) {
        dispatch(fetchMeasurements(stationId));
    }
    else {
        let series = Series.from_data(
            measurements.map(mes => mes.timestamp),
            measurements.map(mes => mes.value),
        );

        // FIXME: batch evaluate all timestamps
        const predictions = measurements
            .map(entry => series.evaluate(entry.timestamp));

        return <table>
            {
                measurements.map((mes, i) => {
                    return <tr>
                        <td>{Number(mes.timestamp)}</td>
                        <td>{mes.value}</td>
                        <td>{predictions[i]}</td>
                    </tr>;
                })
            }
        </table>;
    }

    return <div/>;
}
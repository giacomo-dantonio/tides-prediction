/* global BigInt */

import React from 'react';
import { useSelector, useDispatch } from 'react-redux';

import {Series} from 'tides-signals';

import {fetchMeasurements, selectMeasurements, FETCH_STATE} from './measurementsSlice';

export default function Station(props) {
    const {stationId} = props;
    const dispatch = useDispatch();

    // FIXME show anymations for fetching and computing states
    const {fetching, value: measurements} = useSelector(selectMeasurements);
    if (fetching === FETCH_STATE.INITIAL) {
        dispatch(fetchMeasurements(stationId));
    }
    else if (fetching === FETCH_STATE.FETCHED) {
        let series = Series.from_data(
            measurements.map(mes => BigInt(mes.timestamp)),
            measurements.map(mes => mes.value),
        );

        // FIXME: batch evaluate all timestamps
        const predictions = measurements
            .map(entry => series.evaluate(BigInt(entry.timestamp)));

        return <table>
            <tbody>
            {
                measurements.map((mes, i) => {
                    return <tr key={Number(mes.timestamp)}>
                        <td>{Number(mes.timestamp)}</td>
                        <td>{mes.value}</td>
                        <td>{predictions[i]}</td>
                    </tr>;
                })
            }
            </tbody>
        </table>;
    }

    return <div/>;
}
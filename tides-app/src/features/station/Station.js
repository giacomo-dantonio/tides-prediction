/* global BigInt */

import React from 'react';
import {useSelector, useDispatch} from 'react-redux';

import {Series} from 'tides-signals';

import {
    fetchMeasurements,
    selectMeasurements,
    FETCH_STATE
} from './measurementsSlice';

import {
    batchSet,
    selectPredictions
} from './predictionsSlice';

export default function Station(props) {
    const {stationId} = props;
    const dispatch = useDispatch();

    // FIXME show anymations for fetching and computing states
    const {fetching, value: measurements} = useSelector(selectMeasurements);
    const predictions = useSelector(selectPredictions);

    if (fetching === FETCH_STATE.INITIAL) {
        dispatch(fetchMeasurements(stationId));
    }
    else if (fetching === FETCH_STATE.FETCHED) {
        // FIXME: find out a better way to check if predictions have
        // already be computed
        if (Object.keys(predictions).length === 0)
        {
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

        return <table>
            <tbody>
            {
                measurements.map((mes, i) => {
                    return <tr key={mes.timestamp}>
                        <td>{mes.timestamp}</td>
                        <td>{mes.value}</td>
                        <td>{predictions[mes.timestamp]}</td>
                    </tr>;
                })
            }
            </tbody>
        </table>;
    }

    return <div/>;
}
import React from 'react';
import {useSelector, useDispatch} from 'react-redux';

import {FETCH_STATE, fetchStations, selectStations} from './stationsSlice';
import {selectStation, setStation} from '../station/selectedStationSlice';
import {clearAndThen} from '../utils/actions';

const capitalize = name => name.charAt(0).toUpperCase() + name.slice(1).toLowerCase();

export default function StationChooser(props) {
    const dispatch = useDispatch();

    const {fetching, value: stations} = useSelector(selectStations);
    const stationId = useSelector(selectStation);

    if (fetching === FETCH_STATE.INITIAL) {
        dispatch(fetchStations());
    }

    return <div className="station-dropdown">
        <label className="label" htmlFor="station-select">Station</label>
        <select name="stations" id="station-select" value={stationId}>
            {
            stations.map(station =>
                <option
                    key={station.uuid}
                    value={station.uuid}
                    onClick={() => dispatch(clearAndThen(setStation(station.uuid)))}
                    readOnly
                >{capitalize(station.longname)}</option>
            )
            }
        </select>
    </div>;
}
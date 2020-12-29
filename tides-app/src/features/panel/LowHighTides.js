import React from 'react';
import {useSelector, useDispatch} from 'react-redux';

import {select as selectExtremes, set as setExtremes} from '../station/extremesSlice';

function dateStr(timestamp)
{
    const pad = n => n.toString().padStart("2", "0");

    const d = new Date(timestamp * 1000);
    const date = pad(d.getDate());
    const month = pad(d.getMonth() + 1);
    const year = d.getFullYear();
    const hours = pad(d.getHours());
    const minutes = pad(d.getMinutes());

    return `${date}.${month}.${year} ${hours}:${minutes}`;
}

function Entry(props) {
    const {timestamp, level, className, label} = props;
    return <div className={className}>
        <div className="label">{label}</div>
        <table>
            <tbody>
                <tr>
                    <td className="label">At</td>
                    <td>{dateStr(timestamp)}</td>
                </tr>
                <tr>
                    <td className="label">Level</td>
                    <td>{level.toFixed(2)}</td>
                </tr>
            </tbody>
        </table>
    </div>;
}

function LowTideEntry(props)
{
    return <Entry
        className="low-tide-entry"
        label="Low tide"
        {...props}
    />;
}

function HighTideEntry(props)
{
    return <Entry
        className="high-tide-entry"
        label="High tide"
        {...props}
    />;
}

export default function LowHightTides() {
    const {minima, maxima} = useSelector(selectExtremes);
    const data = minima.map(minimum => ({
        ...minimum,
        type: "low"
    }));
    data.push.apply(data, maxima.map(maximum => ({
        ...maximum,
        type: "high"
    })));

    let children = minima.map(minimum => 
        <LowTideEntry
            key={minimum.timestamp}
            timestamp={minimum.timestamp}
            level={minimum.value}
        />);

    children.push.apply(children, maxima.map(maximum => 
        <HighTideEntry
            key={maximum.timestamp}
            timestamp={maximum.timestamp}
            level={maximum.value}
        />));

    children.sort((lhs, rhs) => lhs.props.timestamp - rhs.props.timestamp);

    return <div className="low-high-tides-panel">
        {children}
    </div>
}

import React from 'react';
import {useSelector} from 'react-redux';

import {select} from '../../slices/loadingSlice';
import waves from './waves.svg';

export default function Loading() {
    const loading = useSelector(select);

    if (loading.length == 0) {
        return <div/>;
    }

    return <div className="loading-panel">
        <div className="loading-messages">
            {loading.map(({key, message}) =>
                <div key={key}>{message}</div>
            )}
        </div>
        <img className="loading-waves" src={waves}/>
    </div>
}

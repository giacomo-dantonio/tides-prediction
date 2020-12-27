import React, { useEffect, useState } from 'react';
import Chart from 'chart.js';

export default function ChartComponent(props) {
    const canvasId = props.canvasId || "chart-canvas";
    const canvasLabel = props.label || "Data chart";

    // initialize timeRange to 3 days (in minutes)
    const [timeRange, setTimeRange] = useState({
        center: Date.now(),
        width: 3 * 24 * 60
    });
    const [chart, setChart] = useState(null);
    const [panning, setPanning] = useState(false);

    const eventCallback = event => {
        switch (event.type) {
        case "wheel":
            setTimeRange({
                ...timeRange,
                width: timeRange.width - 1E2 * event.deltaY
            });
            break;
        case "mousedown":
            console.log("mousedown");
            setPanning(true);
            break;
        case "mouseup":
            console.log("mouseup");
            setPanning(false);
            break;
        case "mousemove":
            if (panning) {
                console.log("mousemove", event.movementX, event.offsetX);
                setTimeRange({
                    ...timeRange,
                    center: timeRange.center + 1E6 * event.movementX
                });
            }
            break;
        }
    };

    // Only create the chart once after first rendering,
    // and update it every time the data change.
    useEffect(() => {
        const options = makeOptions({timeRange, eventCallback});
        const datasets = makeDatasets(props.series);
        const config = {
            type: 'line',
            data: {
                datasets
            },
            options
        };
    
        const ctx = document.getElementById(canvasId).getContext('2d');
        setChart(new Chart(ctx, config));
    }, []);

    useEffect(() => {
        if (chart !== null) {
            chart.options.onHover = eventCallback;
            chart.options.scales.xAxes[0].ticks = makeTicks(timeRange);
            chart.update();
        }
    }, [panning, timeRange]);

    useEffect(() => {
        if (chart !== null) {
            chart.config.data.datasets = makeDatasets(props.series);
            chart.update();
        }
    }, props.series);

    return <div>
        <canvas
            id={canvasId}
            aria-label={canvasLabel}
            role="img"
        />
    </div>;
}

function makeOptions(args) {
    return Object.assign({
        hover: {
            animationDuration: 0
        },
        events: [
            "mousedown",
            "mousemove",
            "mouseup",
            "mouseout",
            "click",
            "touchstart",
            "touchmove",
            "wheel"
        ],
        onHover: args.eventCallback,
        scales: {
            xAxes: [{
                type: "time",
                time: {
                    unit: "hour",
                    displayFormats: {
                        hour: "D MMM YYYY H:mm",
                    }
                },
                ticks : makeTicks(args.timeRange)
            }],
            yAxes: [{
                id: "pegel-axis",
                gridLines: {
                    display: false
                }
            }],
        }
    });
}

function makeDatasets(series) {
    const datasets = [];
    for (const dataSeries of (series || [])) {
        const dataset = {
            label: dataSeries.name,
            backgroundColor: dataSeries.color,
            borderColor: dataSeries.color,
            fill: false,
            yAxisID: "pegel-axis",
            data: dataSeries.values.map(entry => ({
                x: new Date(entry.timestamp * 1000),
                y: entry.value
            }))
        };
        datasets.push(dataset);
    }
    return datasets;
}

function makeTicks(timeRange) {
    return {
        min: new Date(timeRange.center - timeRange.width * 60 * 1000),
        max: new Date(timeRange.center + timeRange.width * 60 * 1000),
        sampleSize: 7,
    };
}

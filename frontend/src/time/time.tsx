import React, { Component } from 'react';

const URL = 'ws://localhost:8081/ws/time';

interface TimeProperties {

}

interface TimeState {
    time?: Date,
    ws?: WebSocket,
}

class TimeComponent extends Component<TimeProperties, TimeState> {
    constructor(props: TimeProperties) {
        super(props);

        this.state = {
            time: undefined,
            ws: undefined,
        };
    }

    componentDidMount() {
        let ws = new WebSocket(URL, 'time');

        ws.onopen = () => {
            console.log('WS CONNECTED')

            /*setInterval(() => {
                ws.send("test")
            }, 1000);*/
        };

        ws.onmessage = event => {
            console.log("RECEIVED " + event.data);

            this.setState({
                time: event.data
            });
        };

        ws.onclose = () => {
            console.log('WS DISCONNECTED');

            this.setState({
                ws: new WebSocket(URL, 'time'),
            });
        };

        this.setState({
            ws: ws
        });
    }

    testResponse() {
        this.state.ws!.send("foo");
    }

    render(): JSX.Element {
        return <>
            <p>{this.state.time}</p>
            <button onClick={() => this.testResponse()}>Test</button>
        </>
    }
}

export default TimeComponent;

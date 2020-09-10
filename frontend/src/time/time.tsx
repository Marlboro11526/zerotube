import React, { Component } from "react";

const URL = "wss://localhost:8443/ws/time";

let isClosing = false;
let ws: WebSocket;

interface TimeProperties {

}

interface TimeState {
    time?: Date,
}

export default class Time extends Component<TimeProperties, TimeState> {
    constructor(props: TimeProperties) {
        super(props);

        this.state = {
            time: undefined,
        };
    }

    componentDidMount(): void {
        ws = new WebSocket(URL, "time");

        ws.onmessage = event => {
            this.setState({
                time: event.data
            });
        };

        ws.onclose = () => {
            if (!isClosing) {
                setTimeout(
                    () => ws = new WebSocket(URL),
                    5000
                );
            }
        };
    }

    componentWillUnmount(): void {
        isClosing = true;
        ws.close();
    }

    testResponse(): void {
        ws.send("foo");
    }

    render(): JSX.Element {
        return <>
            <p>{this.state.time}</p>
            <button onClick={() => this.testResponse()}>Test</button>
        </>;
    }
}

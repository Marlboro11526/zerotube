import React, { Component } from "react";

export default class Error404 extends Component {
    render(): JSX.Element {
        return (
            <>
                <h1>404 Not Found</h1>
                <p><a href="/">Home</a></p>
            </>
        );
    }
}

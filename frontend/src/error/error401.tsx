import React, { Component } from "react";

export default class Error401 extends Component {
    render(): JSX.Element {
        return (
            <>
                <h1>401 Unauthorised</h1>
                <p><a href="/">Home</a></p>
            </>
        )
    }
}

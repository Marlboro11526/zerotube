import React, { Component } from "react";
import validator from "validator";

import Error404 from "../error/error404";

interface ConfirmRegisterProperties { }

interface ConfirmRegisterState {
    id?: string,
    isValid?: boolean,
}

export default class ConfirmRegister extends Component<ConfirmRegisterProperties, ConfirmRegisterState> {
    constructor(props: ConfirmRegisterProperties) {
        super(props);

        const id: string = window.location.pathname.slice("/confirm/".length);

        this.state = {
            id: validator.isUUID(id) ? id : undefined,
        };
    }

    async componentDidMount(): Promise<void> {
        if (this.state.id === undefined) {
            return;
        }

        const response = await fetch("https://localhost:8443/auth/register/" + this.state.id, {
            method: "GET",
            credentials: "include",
        });

        this.setState({ isValid: response.ok });
    }

    render(): JSX.Element {
        if (this.state.isValid) {
            return (
                <>
                    <h1>Thanks! You can now log in to ZeroTube.</h1>
                    <p><a href="/">Home</a></p>
                </>
            );
        } else if (this.state.isValid === undefined) {
            return <></>;
        } else {
            return <Error404 />;
        }
    }
}
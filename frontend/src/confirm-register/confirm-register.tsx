import React, { Component } from "react";
import Error404 from "../error/error404";
import validator from "validator";

interface ConfirmRegisterProperties { }

interface ConfirmRegisterState {
    id?: string,
    isValid?: boolean,
}

export default class ConfirmRegister extends Component<ConfirmRegisterProperties, ConfirmRegisterState> {
    constructor(props: ConfirmRegisterProperties) {
        super(props);

        let id: string = window.location.pathname.slice("/confirm/".length);

        this.state = {
            id: validator.isUUID(id) ? id : undefined,
        };
    }

    componentDidMount(): void {
        if (this.state.id === undefined) {
            return;
        }

        fetch("http://localhost:8081/auth/register/" + this.state.id, {
            method: "GET",
            credentials: "include",
        })
            .then(response => {
                this.setState({ isValid: response.ok });
            });
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
            return <></>
        } else {
            return <Error404 />
        }
    }
}
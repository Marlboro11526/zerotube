import React, { Component, ChangeEvent } from "react";
import { Input, Label } from "trunx";

interface LoginFormProperties {
    inputHandler: any,
}

interface LoginFormState {
    password?: string,
    username?: string,
}

export default class LoginForm extends Component<LoginFormProperties, LoginFormState> {
    constructor(props: LoginFormProperties) {
        super(props);

        this.state = {
            password: undefined,
            username: undefined,
        };

        this.handleInputChange = this.handleInputChange.bind(this);
    }

    componentWillUnmount(): void {
        this.props.inputHandler(undefined, undefined);
    }

    handleInputChange(event: ChangeEvent<HTMLInputElement>): void {
        const target = event.target!;
        const name = target.name;
        const value = target.type === "checkbox" ? target.checked : target.value;

        this.setState({
            [name]: value,
        } as Pick<LoginFormState, any>,
            () => this.props.inputHandler(this.state.password, this.state.username));
    }

    render(): JSX.Element {
        return (
            <>
                <Label>Username:</Label>
                <Input
                    name="username"
                    type="text"
                    onChange={this.handleInputChange}
                    autoFocus />
                <Label>Password:</Label>
                <Input
                    name="password"
                    type="password"
                    onChange={this.handleInputChange} />
            </>
        );
    }
}

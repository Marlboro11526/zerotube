import React, { Component, ChangeEvent } from "react";
import { Input, Label, Field } from "trunx";

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
        const TARGET = event.target;
        const NAME = TARGET.name;
        const VALUE = TARGET.type === "checkbox" ? TARGET.checked : TARGET.value;

        this.setState({
            [NAME]: VALUE,
        } as LoginFormState,
            () => this.props.inputHandler(this.state.password, this.state.username));
    }

    render(): JSX.Element {
        return (
            <>
                <Field>
                    <Label>Username</Label>
                    <Input
                        name="username"
                        type="text"
                        onChange={this.handleInputChange}
                        autoFocus />
                </Field>
                <Field>
                    <Label>Password</Label>
                    <Input
                        name="password"
                        type="password"
                        onChange={this.handleInputChange} />
                </Field>
            </>
        );
    }
}

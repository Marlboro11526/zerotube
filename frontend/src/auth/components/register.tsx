import React, { Component, ChangeEvent } from "react";
import { Label, Input, Field } from "trunx";
import Icon from "@mdi/react";
import { mdiHelpCircle } from "@mdi/js";

interface RegisterFormProperties {
    inputHandler: any,
}

interface RegisterFormState {
    email?: string,
    password?: string,
    username?: string,
}

export default class RegisterForm extends Component<RegisterFormProperties, RegisterFormState> {
    constructor(props: RegisterFormProperties) {
        super(props);

        this.state = {
            email: undefined,
            password: undefined,
            username: undefined,
        };

        this.handleInputChange = this.handleInputChange.bind(this);
    }

    componentWillUnmount(): void {
        this.props.inputHandler(undefined, undefined);
    }

    handleInputChange(event: ChangeEvent<HTMLInputElement>): void {
        const target = event.target;
        const name = target.name;
        const value = target.type === "checkbox" ? target.checked : target.value;

        this.setState({
            [name]: value,
        } as RegisterFormState,
            () => this.props.inputHandler(this.state.email, this.state.password, this.state.username));
    }

    render(): JSX.Element {
        return (
            <>
                <Field>
                    <Label>
                        Email
                        <Icon
                            color="#FFF"
                            path={mdiHelpCircle}
                            size={"1rem"}
                            title="Needs to be valid so that you can click the confirmation link!"
                        />
                    </Label>
                    <Input
                        name="email"
                        type="text"
                        onChange={this.handleInputChange}
                        autoFocus
                    />
                </Field>
                <Field>
                    <Label>Username</Label>
                    <Input
                        name="username"
                        type="text"
                        onChange={this.handleInputChange}
                    />
                </Field>
                <Field>
                    <Label>Password</Label>
                    <Input
                        name="password"
                        type="password"
                        onChange={this.handleInputChange}
                    />
                </Field>
            </>
        );
    }
}
